use axum::http::StatusCode;
use axum::response::Response;
use axum::{extract, response::Json};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::dance_activity::NewDanceActivity;
use crate::user::UserId;
use crate::{internal_error, AppState};

#[derive(serde::Serialize)]
pub(crate) struct ClientSessionResponse {
    client_session_id: i64,
    client_session_secret: Uuid,
}

#[derive(serde::Deserialize)]
pub(crate) struct RecordActivityRequest {
    client_session_id: i64,
    client_session_secret: Uuid,

    activity_id: String,
    recorded_at: DateTime<Utc>,
    hits: i32,
    misses: i32,
    steps: i32,
}

#[derive(Clone)]
/// Authenticated client session
pub struct ClientSessionId(i64);

/// Public API to create a new guest session.
pub(crate) async fn create_guest_session(
    extract::State(state): extract::State<AppState>,
) -> Result<Json<ClientSessionResponse>, (StatusCode, String)> {
    let client_session_secret = Uuid::new_v4();

    let user_id = UserId::create_new_guest(&state.pg_db_pool).await;

    let result: Option<(i64,)> = sqlx::query_as(
        "INSERT INTO client_session (client_session_secret, user_id) VALUES ($1, $2) RETURNING id",
    )
    .bind(client_session_secret)
    .bind(user_id.num())
    .fetch_optional(&state.pg_db_pool)
    .await
    .map_err(internal_error)?;

    if let Some((client_session_id,)) = result {
        Ok(Json(ClientSessionResponse {
            client_session_id,
            client_session_secret,
        }))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "DB failed inserting new session".to_owned(),
        ))
    }
}

/// (WIP) API to add an activity to a guest session.
pub(crate) async fn record_guest_activity(
    extract::State(state): extract::State<AppState>,
    Json(payload): Json<RecordActivityRequest>,
) -> Response {
    let session = ClientSessionId::authenticate_guest_session(
        &state.pg_db_pool,
        payload.client_session_id,
        &payload.client_session_secret,
    )
    .await;

    match session {
        Some(client_session_id) => {
            let result = NewDanceActivity {
                client_session_id: client_session_id.0,
                activity_id: payload.activity_id,
                recorded_at: payload.recorded_at.naive_utc(),
                hits: payload.hits,
                misses: payload.misses,
                steps: payload.steps,
            }
            .store_to_db(&state.pg_db_pool)
            .await;

            if let Err(err) = result {
                tracing::warn!("failed inserting a new activity, {err:?}");

                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("failed inserting a new activity".into())
                    .expect("response builder should succeed")
            } else {
                Response::builder()
                    .status(StatusCode::OK)
                    .body("OK".into())
                    .expect("response builder should succeed")
            }
        }
        None => Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body("failed session authentication".into())
            .expect("response builder should succeed"),
    }
}

impl ClientSessionId {
    // Authenticate a guest client session by its secret.
    pub async fn authenticate_guest_session(
        db_pool: &PgPool,
        client_session_id: i64,
        secret: &Uuid,
    ) -> Option<Self> {
        match sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM client_session
                WHERE id = $1 AND client_session_secret = $2
            )
            "#,
            client_session_id,
            secret
        )
        .fetch_one(db_pool)
        .await
        {
            Ok(Some(true)) => Some(ClientSessionId(client_session_id)),
            Ok(Some(false)) => {
                info!(
                    client_session_id,
                    "Guest session authentication failed (invalid secret)"
                );

                let Ok(session_exists) = sqlx::query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM client_session WHERE id = $1 OR client_session_secret = $2)",
                    client_session_id,
                    secret
                ).fetch_one(db_pool)
                .await else {
                    error!(
                        client_session_id,
                        "Database error during guest session lookup"
                    );
                    return None;
                };

                if !session_exists.unwrap_or(true) {
                    create_client_session_from_external(db_pool, client_session_id, secret).await
                } else {
                    None
                }
            }
            Ok(None) => {
                error!(client_session_id, "Database returned None for some reason");
                None
            }
            Err(e) => {
                error!(
                    client_session_id,
                    error = %e,
                    "Database error during guest session authentication"
                );
                None
            }
        }
    }
}

/// Allows creating a client session that has been created by offline clients,
/// or previous versions where I lost the data.
async fn create_client_session_from_external(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    client_session_id: i64,
    secret: &Uuid,
) -> Option<ClientSessionId> {
    match sqlx::query!(
        "INSERT INTO client_session (id, client_session_secret) 
                                         VALUES ($1, $2) 
                                         ON CONFLICT (id) DO NOTHING RETURNING id",
        client_session_id,
        secret
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(_)) => Some(ClientSessionId(client_session_id)),
        Ok(None) => {
            error!(
                client_session_id,
                "Failed to create new guest session (conflict or other issue)"
            );
            None
        }
        Err(e) => {
            error!(client_session_id, error = %e, "Database error during guest session creation");
            None
        }
    }
}

impl ClientSessionId {
    pub fn num(&self) -> i64 {
        self.0
    }
}
