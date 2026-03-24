use crate::user::User;
use crate::{internal_error, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, Debug)]
pub struct UpdateMetaDataRequest {
    pub(crate) key_name: String,
    pub(crate) key_value: String,
    pub(crate) last_modified: DateTime<Utc>,
    pub(crate) version: u16,
}

#[derive(serde::Serialize, Debug)]
enum UpdateMetaDataErrorCode {
    InvalidKeyName = 1,
    InvalidKeyValue = 2,
    UnexpectedVersion = 3,
    NewerValueExists = 4,
    InternalServerError = 5,
}

#[derive(serde::Serialize, Debug)]
pub struct UpdateMetaDataError {
    code: UpdateMetaDataErrorCode,
    message: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserMeta {
    key_name: String,
    key_value: String,
    last_modified: Option<chrono::NaiveDateTime>,
    version_nr: i16,
}
#[axum::debug_handler]
/// Read all the user metadata for the authenticated user
pub async fn metadata(Extension(user): Extension<User>, State(state): State<AppState>) -> Response {
    let result = sqlx::query_as!(
        UserMeta,
        r#"
        SELECT key_name, key_value, last_modified, version_nr
        FROM user_meta
        WHERE user_id = $1
        "#,
        user.id.num()
    )
    .fetch_all(&state.pg_db_pool)
    .await;

    match result {
        Ok(user_meta) => (StatusCode::OK, Json(user_meta)).into_response(),
        Err(e) => internal_error(e).into_response(),
    }
}
#[axum::debug_handler]
/// Update the user metadata for the authenticated user
pub async fn update_user_metadata(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateMetaDataRequest>,
) -> Result<(StatusCode, &'static str), (StatusCode, Json<UpdateMetaDataError>)> {
    let key_name = payload.key_name.trim();
    let key_value = payload.key_value;
    let new_last_modified = payload.last_modified.naive_utc();
    let version = i16::try_from(payload.version).unwrap_or(i16::MAX);

    if key_name.is_empty() || key_name.len() > 64 {
        return Err((
            StatusCode::BAD_REQUEST,
            UpdateMetaDataError::json(
                UpdateMetaDataErrorCode::InvalidKeyName,
                "Invalid key_name. Must be non-empty and up to 64 characters.",
            ),
        ));
    }

    if key_value.is_empty() || key_value.len() > 1024 {
        return Err((
            StatusCode::BAD_REQUEST,
            UpdateMetaDataError::json(
                UpdateMetaDataErrorCode::InvalidKeyValue,
                "Invalid key_name. Must be non-empty and up to 1024 characters.",
            ),
        ));
    }

    // This version check is for the future, in case fields or the general
    // format need to be updated. For now, there is only one version, so there
    // should only be requests matching that version.
    if version != 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            UpdateMetaDataError::json(
                UpdateMetaDataErrorCode::UnexpectedVersion,
                format!("Unexpected data version {version}."),
            ),
        ));
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO user_meta (user_id, key_name, key_value, last_modified, version_nr)
        VALUES($1, $2, $3, $4, $5)
        ON CONFLICT (user_id, key_name)
        DO UPDATE SET
            key_value = EXCLUDED.key_value,
            last_modified = EXCLUDED.last_modified,
            version_nr = EXCLUDED.version_nr
        WHERE
            user_meta.last_modified < EXCLUDED.last_modified
        "#,
        user.id.num(),
        key_name,
        key_value,
        new_last_modified,
        version,
    )
    .execute(&state.pg_db_pool)
    .await;

    let query_result = result.map_err(|err| {
        tracing::error!(?err, "DB error in update_user_metadata");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            UpdateMetaDataError::json(UpdateMetaDataErrorCode::InternalServerError, "DB error"),
        )
    })?;
    if query_result.rows_affected() == 0 {
        Err((
            StatusCode::CONFLICT,
            UpdateMetaDataError::json(
                UpdateMetaDataErrorCode::NewerValueExists,
                "Update rejected, a newer value for the key exists.",
            ),
        ))
    } else {
        Ok((StatusCode::OK, "User metadata updated successfully."))
    }
}

impl UpdateMetaDataError {
    fn new(code: UpdateMetaDataErrorCode, message: String) -> Self {
        Self { code, message }
    }

    fn json(code: UpdateMetaDataErrorCode, message: impl Into<String>) -> Json<Self> {
        Json(Self::new(code, message.into()))
    }
}
