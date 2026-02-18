use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    combo::{Combo, ComboId},
    timestamp::{Timestamp, TimestampId},
    user::{User, UserId},
    AppState,
};

pub type JsonResponse<T> = axum::response::Result<Json<T>, (StatusCode, &'static str)>;

#[derive(serde::Serialize)]
pub struct CombosResponse {
    combos: Vec<ComboInfo>,
}

#[derive(serde::Deserialize)]
pub struct CreateComboRequest {
    pub is_private: bool,
    pub sort_order: Option<i32>,
    pub free_form_category: Option<String>,
    pub title: Option<String>,
    pub video_short_uuid: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct ComboInfo {
    pub id: ComboId,
    pub is_private: bool,
    pub sort_order: Option<i32>,
    pub free_form_category: Option<String>,
    pub title: Option<String>,
    pub video_short_uuid: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct ComboTimestampInfos {
    pub ms: Vec<i32>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct ComboTimestampInfo {
    pub ms: i32,
}

impl UserId {
    pub async fn combos(&self, state: &AppState) -> Result<Vec<Combo>, sqlx::Error> {
        Combo::list_by_user(state, *self).await
    }
}

pub async fn create_combo(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<CreateComboRequest>,
) -> JsonResponse<ComboInfo> {
    let result = Combo::create(
        &state,
        user.id,
        payload.is_private,
        payload.sort_order,
        payload.free_form_category.as_deref(),
        payload.title.as_deref(),
        payload.video_short_uuid.as_deref(),
    )
    .await;

    let Ok(created) = result else {
        let err = result.unwrap_err();
        tracing::error!(?err, "Failed creating combo");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed creating combo"));
    };

    Ok(Json(ComboInfo::from_db_info(created)))
}

pub async fn update_combo(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<ComboInfo>,
) -> JsonResponse<ComboInfo> {
    let result: Result<Option<Combo>, sqlx::Error> =
        Combo::update_combo(&state, &user, payload).await;

    let Ok(maybe_updated) = result else {
        let err = result.unwrap_err();
        tracing::error!(?err, "Failed updating combo");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed updating combo"));
    };

    let Some(updated) = maybe_updated else {
        return Err((StatusCode::UNAUTHORIZED, "Combo belongs to another user"));
    };

    Ok(Json(ComboInfo::from_db_info(updated)))
}

pub async fn user_combos(
    State(state): State<AppState>,
    Path(user_id): Path<UserId>,
) -> JsonResponse<CombosResponse> {
    let result = user_id.combos(&state).await;

    let Ok(db_combos) = result else {
        let err = result.unwrap_err();
        tracing::error!(?err, ?user_id, "Failed fetching combos of user");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed fetching combos"));
    };

    let combos = db_combos
        .into_iter()
        .filter(|c| !c.is_private)
        .map(ComboInfo::from_db_info)
        .collect();

    Ok(Json(CombosResponse { combos }))
}

pub async fn combo_timestamps(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(combo_id): Path<ComboId>,
) -> JsonResponse<ComboTimestampInfos> {
    // TODO: public combos should be visible without login
    assert_owns_combo(&state, user.id, combo_id).await?;

    let result = Timestamp::list_by_combo(&state, combo_id).await;

    let Ok(timestamps) = result else {
        let err = result.unwrap_err();
        tracing::error!(?err, ?combo_id, "Failed fetching combo timestamps");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed fetching combo timestamps",
        ));
    };

    Ok(Json(ComboTimestampInfos {
        ms: timestamps.into_iter().map(|t| t.milliseconds).collect(),
    }))
}

pub async fn add_combo_timestamp(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(combo_id): Path<ComboId>,
    Json(payload): Json<ComboTimestampInfo>,
) -> axum::response::Result<()> {
    assert_owns_combo(&state, user.id, combo_id).await?;

    let result = Timestamp::create_for_combo(&state, combo_id, payload.ms).await;

    if let Err(err) = result {
        tracing::error!(?err, "Failed creating timestamp for combo");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed creating timestamp for combo",
        ))?;
    };

    Ok(())
}

pub async fn delete_combo_timestamp(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path((combo_id, timestamp_id)): Path<(ComboId, TimestampId)>,
) -> axum::response::Result<()> {
    assert_owns_combo(&state, user.id, combo_id).await?;
    assert_ts_belongs_to_combo(&state, combo_id, timestamp_id).await?;

    let result = Timestamp::delete(&state, timestamp_id).await;

    if let Err(err) = result {
        tracing::error!(?err, "Failed deleting timestamp for combo");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed creating timestamp for combo",
        ))?;
    };

    Ok(())
}

async fn assert_owns_combo(
    state: &AppState,
    user_id: UserId,
    combo_id: ComboId,
) -> Result<(), (StatusCode, &'static str)> {
    let result = Combo::is_owned_by(state, combo_id, user_id).await;
    match result {
        Ok(true) => Ok(()),
        Ok(false) => Err((StatusCode::NOT_FOUND, "combo not found for user")),
        Err(err) => {
            tracing::error!(?err, "Failed looking up combo ownership");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed looking up combo ownership",
            ))
        }
    }
}

async fn assert_ts_belongs_to_combo(
    state: &AppState,
    combo_id: ComboId,
    timestamp_id: TimestampId,
) -> Result<(), (StatusCode, &'static str)> {
    let result = Timestamp::combo_of_timestamp(state, timestamp_id).await;
    match result {
        Ok(Some(c)) => {
            if c.num() == combo_id.num() {
                Ok(())
            } else {
                Err((StatusCode::NOT_FOUND, "combo not found for user"))
            }
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, "combo not found for user")),
        Err(err) => {
            tracing::error!(?err, "Failed looking up combo ownership");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed looking up combo ownership",
            ))
        }
    }
}

impl ComboInfo {
    fn from_db_info(other: Combo) -> ComboInfo {
        ComboInfo {
            id: other.id,
            is_private: other.is_private,
            sort_order: other.sort_order,
            free_form_category: other.free_form_category,
            title: other.title,
            video_short_uuid: other.video_short_uuid,
        }
    }
}
