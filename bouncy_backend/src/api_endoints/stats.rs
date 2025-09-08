use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sqlx::PgPool;

use crate::{internal_error, AppState};

#[derive(serde::Serialize)]
struct UsageStats {
    users: u64,
    client_sessions: u64,
    activities: u64,

    hits: u64,
    steps: u64,
}

#[axum::debug_handler]
/// Return public usage statistics
pub async fn stats(State(state): State<AppState>) -> Response {
    let result = fetch_usage_data(&state.pg_db_pool).await;

    match result {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(e) => internal_error(e).into_response(),
    }
}

async fn fetch_usage_data(db: &PgPool) -> sqlx::Result<UsageStats> {
    let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await?;

    let client_sessions: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM client_session")
        .fetch_one(db)
        .await?;

    let activities: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM dance_activity")
        .fetch_one(db)
        .await?;

    let hits: i64 = sqlx::query_scalar("SELECT SUM(hits) FROM dance_activity")
        .fetch_one(db)
        .await?;

    let steps: i64 = sqlx::query_scalar("SELECT SUM(steps) FROM dance_activity")
        .fetch_one(db)
        .await?;

    Ok(UsageStats {
        users: users as u64,
        client_sessions: client_sessions as u64,
        activities: activities as u64,
        hits: hits as u64,
        steps: steps as u64,
    })
}
