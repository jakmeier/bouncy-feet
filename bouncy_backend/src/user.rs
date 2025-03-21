//! Replaces sqlite based users.

use axum::extract::{Request, State};
use axum::http::header::GetAll;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use axum_oidc::OidcClaims;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AdditionalClaims;
use crate::client_session::ClientSessionId;
use crate::AppState;

#[derive(Clone)]
pub struct UserId(i64);

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
            // Attach user ID for downstream handlers
            req.extensions_mut().insert(user_id);
        }
        Err(response) => return response,
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
            user_lookup_by_client_secret(state, client_session_id, client_session_secret).await;
        maybe_user.ok_or_else(|| auth_error_response("User not found"))
    } else if let Some(claims) = maybe_claims {
        // this will lazily create the user if necessary
        Ok(user_lookup_by_oidc(state, claims).await)
    } else {
        auth_error("Auth failed")
    }
}

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

async fn user_lookup_by_oidc(state: &AppState, claims: OidcClaims<AdditionalClaims>) -> UserId {
    let subject = claims.subject().as_str();

    let maybe_user = sqlx::query!("SELECT id FROM users WHERE oidc_subject = $1", subject)
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed");

    // Lazy user row creation in DB
    let user_id = match maybe_user {
        Some(user) => user.id,
        None => {
            sqlx::query!(
                r#"
                INSERT INTO users (oidc_subject) 
                VALUES ($1)
                RETURNING id
                "#,
                subject
            )
            .fetch_one(&state.pg_db_pool)
            .await
            .expect("Failed to insert new user")
            .id
        }
    };
    UserId(user_id)
}

pub(crate) async fn user_lookup_by_client_secret(
    state: &AppState,
    client_session_id: i64,
    client_session_secret: Uuid,
) -> Option<UserId> {
    let client_session = ClientSessionId::authenticate_guest_session(
        &state.pg_db_pool,
        client_session_id,
        &client_session_secret,
    )
    .await?;

    let user_id = sqlx::query!(
        "SELECT user_id FROM client_session WHERE id = $1",
        client_session.num()
    )
    .fetch_optional(&state.pg_db_pool)
    .await
    .expect("DB query failed");

    user_id.map(|record| UserId(record.user_id))
}

pub async fn user_info(
    claims: OidcClaims<AdditionalClaims>,
    Extension(UserId(user_id)): Extension<UserId>,
) -> String {
    json!({
        "id": user_id,
        "sub": claims.subject().as_str()
    })
    .to_string()
}

fn auth_error<T>(msg: &'static str) -> Result<T, Response> {
    Err(auth_error_response(msg))
}

fn auth_error_response(msg: &'static str) -> Response {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(msg.into())
        .expect("response builder should succeed")
}

impl UserId {
    pub fn num(&self) -> i64 {
        self.0
    }

    pub(crate) async fn create_new_guest(db: &PgPool) -> Self {
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (oidc_subject) 
            VALUES (null)
            RETURNING id
            "#,
        )
        .fetch_one(db)
        .await
        .expect("Failed to insert new guest user")
        .id;
        UserId(user_id)
    }
}
