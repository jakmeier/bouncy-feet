//! Replaces sqlite based users.

use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use axum_oidc::OidcClaims;
use serde_json::json;

use crate::auth::AdditionalClaims;
use crate::AppState;

#[derive(Clone)]
pub struct UserId(i64);

// Middleware to lookup user or create it lazily
pub async fn user_lookup(
    State(state): State<AppState>,
    claims: OidcClaims<AdditionalClaims>,
    mut req: Request,
    next: Next,
) -> Response {
    let subject = claims.subject().as_str();

    let user_exists = sqlx::query!("SELECT id FROM users WHERE oidc_subject = $1", subject)
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed");

    // Lazy user row creation in DB
    let user_id = match user_exists {
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

    // Attach user ID for downstream handlers
    req.extensions_mut().insert(UserId(user_id));

    next.run(req).await
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
