use crate::layers::oidc::AdditionalClaims;
use crate::user::User;
use crate::AppState;
use axum::extract::{Request, State};
use axum::http::header::GetAll;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_oidc::OidcClaims;
use serde_json::json;
use uuid::Uuid;

// Middleware to lookup user or create it lazily from an OIDC claim.
pub async fn user_lookup(
    State(state): State<AppState>,
    maybe_claims: Option<OidcClaims<AdditionalClaims>>,
    mut req: Request,
    next: Next,
) -> Response {
    let auth_headers = &req.headers().get_all("Authorization");
    match try_get_user(&state, auth_headers, maybe_claims).await {
        Ok(user) => {
            let user_id = &user.id;
            // Add user info to logs
            tracing::Span::current().record("user_id", tracing::field::display(user_id));
            // Attach user for downstream handlers
            req.extensions_mut().insert(user);
        }
        Err(response) => {
            // Continue without user info.
            // Routes that require it will fail.
            // But the login route must work without user info.
            tracing::Span::current().record("user_id", tracing::field::display("?"));
            tracing::debug!(err = ?response);
        }
    }

    next.run(req).await
}

async fn try_get_user(
    state: &AppState,
    auth_headers: &GetAll<'_, HeaderValue>,
    maybe_claims: Option<OidcClaims<AdditionalClaims>>,
) -> Result<User, Response> {
    if let Some((client_session_id, client_session_secret)) =
        client_session_credentials_from_headers(auth_headers)?
    {
        let maybe_user =
            User::lookup_by_client_secret(state, client_session_id, client_session_secret).await;
        let user = maybe_user.ok_or_else(|| auth_error_response("User not found"))?;
        if user.oidc_subject.is_some() && maybe_claims.is_none() {
            // The user has a Keycloak user but is currently trying to log in with the guest credentials.
            // This is not allowed, the user must be forced to log in.
            Err(force_login_response())
        } else {
            Ok(user)
        }
    } else if let Some(claims) = maybe_claims {
        // this will lazily create the user if necessary
        Ok(User::lookup_by_oidc(state, claims).await)
    } else {
        auth_error("No user found")
    }
}

/// Let's the client know that the user was found but authentication through Keycloak is required.
///
/// The client side JS code, when making API requests, will detect these errors. Usually, it will
/// redirect the user to the login page. But it may decide to persist some data first or ask the
/// user before leaving the current page.
///
/// The login always has to go through a browser redirect to `/login` on this server.
fn force_login_response() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [(
            axum::http::header::WWW_AUTHENTICATE,
            // Custom scheme, the client authenticates through an active session, not through a
            // Bearer token as it would be the case with OAuth 2.0.
            HeaderValue::from_static(r#"Session realm="BouncyFeet API", error="login_required""#),
        )],
        Json(json!({
            "error": "login_required",
            "redirect": "/login",
        })),
    )
        .into_response()
}

// TODO: Fix clippy warning
#[allow(clippy::result_large_err)]
fn client_session_credentials_from_headers(
    auth_headers: &GetAll<HeaderValue>,
) -> Result<Option<(i64, Uuid)>, Response> {
    // let headers = req.headers().get_all("Authorization");
    for auth_value in auth_headers {
        // Expected format: "ClientSession 1234:550e8400-e29b-41d4-a716-446655440000"
        let prefix = "ClientSession ";

        let Ok(str_auth_value) = auth_value.to_str() else {
            return auth_error("Authorization header is no string");
        };

        if !str_auth_value.starts_with(prefix) {
            continue;
        }

        let rest = &str_auth_value[prefix.len()..];
        let mut parts = rest.splitn(2, ':');
        let Some(id_str) = parts.next() else {
            return auth_error("Invalid auth format");
        };
        let Some(secret_str) = parts.next() else {
            return auth_error("Invalid auth format");
        };

        let Ok(client_session_id) = id_str.parse::<i64>() else {
            return auth_error("Invalid auth format");
        };

        let Ok(client_session_secret) = Uuid::parse_str(secret_str) else {
            return auth_error("Invalid auth secret format");
        };

        return Ok(Some((client_session_id, client_session_secret)));
    }
    Ok(None)
}

// TODO: Fix clippy warning
#[allow(clippy::result_large_err)]
fn auth_error<T>(msg: &'static str) -> Result<T, Response> {
    Err(auth_error_response(msg))
}

fn auth_error_response(msg: &'static str) -> Response {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(msg.into())
        .expect("response builder should succeed")
}
