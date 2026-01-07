//! User middleware
//!
//! User creation flows:
//!
//! A) Guest into new full user
//!     -> Guest session from /new_guest_activity creates UserId
//!     -> /login adds OIDC sub to existing user, converting to full user
//!     -> client sessions remain unchanged
//! B) Keycloak user is created without client session
//!     -> /login creates a new UserId
//!     -> following calls always use this UserId
//! C) Guest into existing user
//!     -> User has keycloak linked UserID but uses the app without login
//!     -> /new_guest_activity creates UserId
//!     -> /login links client sessions of the guest user with the old user
//!     -> guest session user is marked as deleted
//! D) Log in to wrong account
//!     -> User has keycloak linked UserID and an active client session
//!     -> User login expires
//!     -> User logs in but with a different account
//!     -> TODO: Deal with this gracefully. (E.g. ask user in frontend if switch was intended)

use crate::client_session::ClientSessionId;
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

#[derive(thiserror::Error, Debug, Clone)]
enum UserAuthError {
    #[error("must provide auth information")]
    NoAuthProvided,
    #[error("auth header must be a string")]
    BadAuthHeader,
    #[error("only guests can use client session login")]
    ClientSessionLoginNotAllow,
    #[error("client session belongs to a different user")]
    ClientSessionOfDifferentUser,
    #[error("client session header is not formatted correctly")]
    ClientSessionHeaderMalformed,
    #[error("secret is not a valid Uuid, {0}")]
    ClientSessionSecretMalformed(uuid::Error),
    #[error("user not found")]
    UserNotFound,
    #[error("failed parsing sub UUID, {0}")]
    SubjectParsingFailed(uuid::Error),
    #[error("DB query encountered an error, {0}")]
    DbError(String, &'static str),
}

// Wrapper to differentiate between User handlers that optionally want a User,
// and those that want an explicit MaybeUser returned by user_lookup.
#[derive(Clone, Debug)]
struct MaybeUser(Result<User, UserAuthError>);

/// Middleware to lookup user or create it lazily from an OIDC claim.
///
/// Sets `MaybeUser`.
pub async fn user_lookup(
    State(state): State<AppState>,
    maybe_claims: Option<OidcClaims<AdditionalClaims>>,
    mut req: Request,
    next: Next,
) -> Response {
    let auth_headers = &req.headers().get_all("Authorization");
    let maybe_user = try_get_user(&state, auth_headers, maybe_claims).await;

    // Add user info to logs
    match &maybe_user {
        Ok(user) => {
            let user_id = &user.id;
            tracing::Span::current().record("user_id", tracing::field::display(user_id));
        }
        Err(err) => {
            tracing::Span::current().record("user_id", tracing::field::display("?"));
            tracing::debug!(user_err = ?err);
        }
    }

    // Continue with or without user info.
    // `require_user_service` will catch it as an error later for protected routes.
    // But the login route must work without user info.
    req.extensions_mut().insert(MaybeUser(maybe_user));
    next.run(req).await
}

/// Middleware to enforce a user has been authenticated.
pub async fn require_user_service(mut req: Request, next: Next) -> Response {
    let extensions = req.extensions_mut();
    let Some(MaybeUser(optional_user)) = extensions.get::<MaybeUser>() else {
        tracing::error!("Route requires user but user_lookup didn't run",);
        return internal_error_response("user lookup failed");
    };

    match optional_user {
        Ok(user) => {
            extensions.insert(user.clone());
            next.run(req).await
        }
        Err(e) => e.to_response(),
    }
}

async fn try_get_user(
    state: &AppState,
    auth_headers: &GetAll<'_, HeaderValue>,
    maybe_claims: Option<OidcClaims<AdditionalClaims>>,
) -> Result<User, UserAuthError> {
    if let Some((client_session_id, client_session_secret)) =
        client_session_credentials_from_headers(auth_headers)?
    {
        let maybe_user =
            User::lookup_by_client_secret(state, client_session_id, client_session_secret).await;
        let user = maybe_user.ok_or(UserAuthError::UserNotFound)?;
        // Need to check consistency between user based on client_session and based on claims
        if let Some(user_oidc_sub) = user.oidc_subject {
            validate_existing_oidc_user_matches_claims(
                &maybe_claims,
                user,
                user_oidc_sub.to_string(),
            )
        } else {
            validate_guest_user_matches_claims(state, &maybe_claims, user).await
        }
    } else if let Some(claims) = maybe_claims {
        // Flow B: Keycloak user is created without client session (if necessary)
        // this will lazily create the user if necessary
        Ok(User::lookup_by_oidc_or_create(state, claims).await)
    } else {
        Err(UserAuthError::NoAuthProvided)
    }
}

async fn validate_guest_user_matches_claims(
    state: &AppState,
    maybe_claims: &Option<OidcClaims<axum_oidc::EmptyAdditionalClaims>>,
    mut user: User,
) -> Result<User, UserAuthError> {
    if let Some(claims) = maybe_claims {
        if let Some(existing_user) = User::try_lookup_by_oidc(state, claims).await {
            // Flow C: Guest into existing user
            let res = ClientSessionId::transfer_client_sessions(
                &state.pg_db_pool,
                user.id,
                existing_user.id.clone(),
            )
            .await;
            res.map_err(|err| UserAuthError::db_error(err, "transferring client session failed"))?;
            Ok(existing_user)
        } else {
            // Flow A: Guest into new full user
            let sub = Uuid::parse_str(claims.subject().as_str())
                .map_err(UserAuthError::SubjectParsingFailed)?;
            let db_result = user.add_oidc(state, sub).await;
            db_result.map_err(|err| UserAuthError::db_error(err, "adding oidc to user failed"))?;
            Ok(user)
        }
    } else {
        // this is a normal guest user without OIDC user
        Ok(user)
    }
}

// TODO: Fix clippy warning
#[allow(clippy::result_large_err)]
fn validate_existing_oidc_user_matches_claims(
    maybe_claims: &Option<OidcClaims<axum_oidc::EmptyAdditionalClaims>>,
    user: User,
    existing_user_oidc_sub: String,
) -> Result<User, UserAuthError> {
    if let Some(claims) = maybe_claims {
        if claims.subject().as_str() == existing_user_oidc_sub {
            // This user has a OIDC user and is logged in with the same OIDC user.
            Ok(user)
        } else {
            // Flow D: Log in to wrong account
            // TODO: How to handle this case where an owned session by user A then logs in to user B?
            Err(UserAuthError::ClientSessionOfDifferentUser)
        }
    } else {
        // The user has a Keycloak user but is currently trying to log in with the guest credentials.
        // This is not allowed, the user must be forced to log in.
        tracing::warn!(?user, "Full user tried logging in with guest credentials");
        Err(UserAuthError::ClientSessionLoginNotAllow)
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

fn client_session_credentials_from_headers(
    auth_headers: &GetAll<HeaderValue>,
) -> Result<Option<(i64, Uuid)>, UserAuthError> {
    // let headers = req.headers().get_all("Authorization");
    for auth_value in auth_headers {
        // Expected format: "ClientSession 1234:550e8400-e29b-41d4-a716-446655440000"
        let prefix = "ClientSession ";

        let Ok(str_auth_value) = auth_value.to_str() else {
            return Err(UserAuthError::BadAuthHeader);
        };

        if !str_auth_value.starts_with(prefix) {
            continue;
        }

        let rest = &str_auth_value[prefix.len()..];
        let mut parts = rest.splitn(2, ':');
        let Some(id_str) = parts.next() else {
            return Err(UserAuthError::ClientSessionHeaderMalformed);
        };
        let Some(secret_str) = parts.next() else {
            return Err(UserAuthError::ClientSessionHeaderMalformed);
        };

        let Ok(client_session_id) = id_str.parse::<i64>() else {
            return Err(UserAuthError::ClientSessionHeaderMalformed);
        };

        let client_session_secret =
            Uuid::parse_str(secret_str).map_err(UserAuthError::ClientSessionSecretMalformed)?;

        return Ok(Some((client_session_id, client_session_secret)));
    }
    Ok(None)
}

fn auth_error_response(msg: &'static str) -> Response {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(msg.into())
        .expect("response builder should succeed")
}

fn internal_error_response(msg: &'static str) -> Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(msg.into())
        .expect("response builder should succeed")
}

fn bad_request_response(msg: &'static str) -> Response {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(msg.into())
        .expect("response builder should succeed")
}

impl UserAuthError {
    fn db_error(err: sqlx::Error, msg: &'static str) -> Self {
        UserAuthError::DbError(format!("{err:?}"), msg)
    }

    fn to_response(&self) -> axum::http::Response<axum::body::Body> {
        use UserAuthError::*;
        match self {
            NoAuthProvided => force_login_response(),
            ClientSessionLoginNotAllow => auth_error_response("ClientSessionLoginNotAllow"),
            ClientSessionOfDifferentUser => auth_error_response("ClientSessionOfDifferentUser"),
            BadAuthHeader => bad_request_response("BadAuthHeader"),
            ClientSessionHeaderMalformed => bad_request_response("ClientSessionHeaderMalformed"),
            ClientSessionSecretMalformed(error) => {
                tracing::warn!("ClientSessionSecretMalformed: {error}");
                bad_request_response("ClientSessionSecretMalformed")
            }
            SubjectParsingFailed(error) => {
                tracing::warn!("SubjectParsingFailed: {error}");
                bad_request_response("SubjectParsingFailed")
            }
            UserNotFound => auth_error_response("UserNotFound"),
            DbError(err, msg) => {
                tracing::warn!("{msg}, DbError: {err}");
                internal_error_response("InternalDbError")
            }
        }
    }
}
