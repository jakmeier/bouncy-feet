use crate::layers::oidc::AdditionalClaims;
use crate::user::UserId;
use crate::AppState;
use axum::extract::{Request, State};
use axum::http::header::GetAll;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum_oidc::OidcClaims;
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
        Ok(user_id) => {
            // Add user info to logs
            tracing::Span::current().record("user_id", tracing::field::display(&user_id));
            // Attach user ID for downstream handlers
            req.extensions_mut().insert(user_id);
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
) -> Result<UserId, Response> {
    if let Some((client_session_id, client_session_secret)) =
        client_session_credentials_from_headers(auth_headers)?
    {
        let maybe_user =
            UserId::user_lookup_by_client_secret(state, client_session_id, client_session_secret)
                .await;
        maybe_user.ok_or_else(|| auth_error_response("User not found"))
    } else if let Some(claims) = maybe_claims {
        // this will lazily create the user if necessary
        Ok(UserId::user_lookup_by_oidc(state, claims).await)
    } else {
        auth_error("No user found")
    }
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
