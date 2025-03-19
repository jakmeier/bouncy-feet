//! Replaces sqlite based users.

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use axum_oidc::OidcClaims;
use serde_json::json;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::auth::{AdditionalClaims, ClientSecretAuthPayload};
use crate::client_session::{ClientSessionId, USER_ID_KEY};
use crate::AppState;

#[derive(Clone)]
pub struct UserId(i64);

// Middleware to lookup user or create it lazily from an OIDC claim.
pub async fn user_lookup(
    State(state): State<AppState>,
    maybe_claims: Option<OidcClaims<AdditionalClaims>>,
    session: Session,
    mut req: Request,
    next: Next,
) -> Response {
    println!("user_lookup");

    // TODO: session cookies are sent back for the guest session creation but
    // the browser doesn't even set it. FF and Chromium. Maybe I need to set a domain? Something with cross-origin cookies I assume.

    let user_id = if let Ok(Some(user_id)) = session.get::<i64>(USER_ID_KEY).await {
        UserId(user_id)
    } else if let Some(claims) = maybe_claims {
        // this will lazily create the user if necessary
        let user = user_lookup_by_oidc(state, claims).await;
        session
            .insert(USER_ID_KEY, user.num())
            .await
            .expect("session storage failed");
        user
    } else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Requires user authentication".into())
            .expect("response builder should succeed");
    };

    // Attach user ID for downstream handlers
    req.extensions_mut().insert(user_id);

    next.run(req).await
}

async fn user_lookup_by_oidc(state: AppState, claims: OidcClaims<AdditionalClaims>) -> UserId {
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
    auth: &ClientSecretAuthPayload,
) -> Option<UserId> {
    let client_session =
        ClientSessionId::authenticate_guest_session_from_request(state, auth).await?;

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
