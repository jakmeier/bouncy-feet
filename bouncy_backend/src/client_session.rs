use axum::http::StatusCode;
use axum::response::Response;
use axum::{extract, response::Json};
use sqlx::PgPool;
use tower_sessions::Session;
use tracing::{error, info};
use uuid::Uuid;

use crate::auth::ClientSecretAuthPayload;
use crate::user2::{user_lookup_by_client_secret, UserId};
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
    // TODO
    // activity: DanceActivity,
}

#[derive(Clone)]
/// Authenticated client session
pub struct ClientSessionId(i64);

pub const CLIENT_SESSION_ID_KEY: &str = "CLIENT_SESSION_ID";
pub const USER_ID_KEY: &str = "USER_ID";

/// Public API to create a new guest session.
pub(crate) async fn create_guest_session(
    extract::State(state): extract::State<AppState>,
    session: Session,
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
        session
            .insert(CLIENT_SESSION_ID_KEY, client_session_id)
            .await
            .expect("session storage failed");
        session
            .insert(USER_ID_KEY, user_id.num())
            .await
            .expect("session storage failed");

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

pub(crate) async fn continue_guest_session(
    extract::State(state): extract::State<AppState>,
    session: Session,
    Json(payload): Json<ClientSecretAuthPayload>,
) -> Response {
    let session_id = ClientSessionId::authenticate_guest_session(
        &state.pg_db_pool,
        payload.client_session_id,
        &payload.client_session_secret,
    )
    .await;

    match session_id {
        Some(ClientSessionId(id)) => {
            session
                .insert(CLIENT_SESSION_ID_KEY, id)
                .await
                .expect("session storage failed");

            if let Some(user) = user_lookup_by_client_secret(&state, &payload).await {
                session
                    .insert(USER_ID_KEY, user.num())
                    .await
                    .expect("session storage failed");
                Response::new("OK".into())
            } else {
                Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("No user for that session".into())
                    .expect("response builder should succeed")
            }
        }
        None => Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body("Failed session authentication".into())
            .expect("response builder should succeed"),
    }
}

/// (WIP) API to add an activity to a guest session.
pub(crate) async fn record_guest_activity(
    extract::State(state): extract::State<AppState>,
    Json(payload): Json<RecordActivityRequest>,
) -> Response {
    // TODO: use session key instead
    let session = ClientSessionId::authenticate_guest_session(
        &state.pg_db_pool,
        payload.client_session_id,
        &payload.client_session_secret,
    )
    .await;

    // TODO: save activity, maybe perform some checks, give proper answer
    match session {
        Some(ClientSessionId(id)) => {
            Response::new(format!("WIP endpoint - ok - client session {id}").into())
        }
        None => Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body("WIP endpoint - failed session authentication".into())
            .expect("response builder should succeed"),
    }
}

impl ClientSessionId {
    // pub(crate) async fn authenticate_from_session(session: Session) -> Option<Self> {
    //     let client_session_id = session
    //         .get::<i64>(CLIENT_SESSION_ID_KEY)
    //         .await
    //         .expect("failed reading session key");
    //     client_session_id.map(|id| ClientSessionId(id))
    // }

    pub(crate) async fn authenticate_guest_session_from_request(
        state: &AppState,
        auth: &ClientSecretAuthPayload,
    ) -> Option<Self> {
        Self::authenticate_guest_session(
            &state.pg_db_pool,
            auth.client_session_id,
            &auth.client_session_secret,
        )
        .await
    }

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
                None
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

impl ClientSessionId {
    pub fn num(&self) -> i64 {
        self.0
    }
}
