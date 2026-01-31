use axum::http::StatusCode;
use axum::response::Response;
use axum::Extension;
use axum::{extract, response::Json};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::client_session::ClientSessionId;
use crate::dance_activity::NewDanceActivity;
use crate::user::{User, UserId};
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

/// Protected API to create a new client session for a registered user.
pub(crate) async fn create_client_session(
    extract::State(state): extract::State<AppState>,
    Extension(me): Extension<User>,
) -> Result<Json<ClientSessionResponse>, (StatusCode, String)> {
    let client_session_secret = Uuid::new_v4();

    let user_id = me.id;

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
                client_session_id: client_session_id.num(),
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
