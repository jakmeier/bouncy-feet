use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    beat::{Beat, BeatId},
    combo::{Combo, ComboId},
    db_err_to_status,
    timestamp::{Timestamp, TimestampId},
    user::{User, UserId},
    AppState, CheckedBeatId, CheckedComboId, CheckedTimestampId, CheckedUserId,
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

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct NewBeatInfo {
    pub start: i32,
    pub duration: i32,
    pub bpm: f32,
    pub subbeat_per_move: i16,
    /// gzipped and Base64 encoded `PoseFile`
    pub pose_file: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct BeatInfo {
    pub id: BeatId,
    pub start: i32,
    pub duration: i32,
    pub bpm: f32,
    pub subbeat_per_move: i16,
    /// gzipped and Base64 encoded `PoseFile`
    pub pose_file: Option<String>,
}

impl CheckedUserId {
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

pub async fn delete_combo(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(combo_id): Path<ComboId>,
) -> axum::response::Result<()> {
    let checked_combo_id = CheckedComboId::check_for_user(&state, user.id, combo_id).await?;

    let result = Combo::delete(&state, checked_combo_id).await;

    if let Err(err) = result {
        tracing::error!(?err, "Failed deleting combo");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed deleting combo"))?;
    };

    Ok(())
}

pub async fn public_user_combos(
    State(state): State<AppState>,
    Path(user_id): Path<UserId>,
) -> JsonResponse<CombosResponse> {
    let result = CheckedUserId::Public(user_id).combos(&state).await;

    let Ok(db_combos) = result else {
        let err = result.unwrap_err();
        tracing::error!(?err, ?user_id, "Failed fetching combos of user");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed fetching combos"));
    };

    let combos = db_combos.into_iter().map(ComboInfo::from_db_info).collect();

    Ok(Json(CombosResponse { combos }))
}

pub async fn user_combos(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> JsonResponse<CombosResponse> {
    let result = CheckedUserId::Owned(user.id).combos(&state).await;

    let Ok(db_combos) = result else {
        let err = result.unwrap_err();
        tracing::error!(?err, user_id=?user.id, "Failed fetching combos of user");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed fetching combos"));
    };

    let combos = db_combos.into_iter().map(ComboInfo::from_db_info).collect();

    Ok(Json(CombosResponse { combos }))
}

pub async fn public_combo_timestamps(
    State(state): State<AppState>,
    Path(combo_id): Path<ComboId>,
) -> JsonResponse<ComboTimestampInfos> {
    let checked_id = CheckedComboId::check_anonymous(&state, combo_id).await?;
    combo_timestamps_impl(&state, checked_id).await
}

pub async fn combo_timestamps(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(combo_id): Path<ComboId>,
) -> JsonResponse<ComboTimestampInfos> {
    let checked_id = CheckedComboId::check_for_user(&state, user.id, combo_id).await?;
    combo_timestamps_impl(&state, checked_id).await
}

async fn combo_timestamps_impl(
    state: &AppState,
    checked_combo_id: CheckedComboId,
) -> JsonResponse<ComboTimestampInfos> {
    let result = Timestamp::list_by_combo(state, checked_combo_id).await;

    let Ok(timestamps) = result else {
        let err = result.unwrap_err();
        tracing::error!(?err, ?checked_combo_id, "Failed fetching combo timestamps");
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
    let checked_combo_id = CheckedComboId::check_for_user(&state, user.id, combo_id).await?;
    let result = Timestamp::create_for_combo(&state, checked_combo_id, payload.ms).await;

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
    let checked_combo_id = CheckedComboId::check_for_user(&state, user.id, combo_id).await?;
    let checked_timestamp_id = checked_combo_id
        .check_timestamp(&state, timestamp_id)
        .await?;

    let result = Timestamp::delete(&state, checked_timestamp_id).await;

    if let Err(err) = result {
        tracing::error!(?err, "Failed deleting timestamp for combo");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed creating timestamp for combo",
        ))?;
    };

    Ok(())
}

/// List all beats of a combo, including private combos, if owned by the provided user.
pub async fn combo_beats(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(combo_id): Path<ComboId>,
) -> JsonResponse<Vec<BeatInfo>> {
    let checked_id = CheckedComboId::check_for_user(&state, user.id, combo_id).await?;
    combo_beats_impl(&state, checked_id).await
}

/// List all beats of a combo, if the combo is public.
pub async fn public_combo_beats(
    State(state): State<AppState>,
    Path(combo_id): Path<ComboId>,
) -> JsonResponse<Vec<BeatInfo>> {
    let checked_id = CheckedComboId::check_anonymous(&state, combo_id).await?;
    combo_beats_impl(&state, checked_id).await
}

/// List all beats of a combo.
///
/// Privacy: This lists beats even if the combo itself is private. The caller needs to ensure
async fn combo_beats_impl(
    state: &AppState,
    checked_combo_id: CheckedComboId,
) -> JsonResponse<Vec<BeatInfo>> {
    let result = Beat::list_by_combo(state, checked_combo_id).await;

    let Ok(beats) = result else {
        let err = result.unwrap_err();
        tracing::error!(?err, ?checked_combo_id, "Failed fetching combo beats");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed fetching combo beats",
        ));
    };

    Ok(Json(
        beats
            .into_iter()
            .map(|beat| BeatInfo {
                id: beat.id,
                start: beat.start,
                duration: beat.duration,
                bpm: beat.bpm,
                subbeat_per_move: beat.subbeat_per_move,
                pose_file: beat.pose_file,
            })
            .collect(),
    ))
}

pub async fn add_combo_beat(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(combo_id): Path<ComboId>,
    Json(payload): Json<NewBeatInfo>,
) -> JsonResponse<BeatId> {
    let checked_combo_id = CheckedComboId::check_for_user(&state, user.id, combo_id).await?;

    let result = Beat::create_for_combo(
        &state,
        checked_combo_id,
        payload.start,
        payload.duration,
        payload.bpm,
        payload.subbeat_per_move,
        payload.pose_file,
    )
    .await;

    Ok(Json(result.unwrap().id))
}

pub async fn delete_combo_beat(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path((combo_id, beat_id)): Path<(ComboId, BeatId)>,
) -> axum::response::Result<()> {
    let checked_combo_id = CheckedComboId::check_for_user(&state, user.id, combo_id).await?;
    let checked_beat_id = checked_combo_id.check_beat(&state, beat_id).await?;

    let result = Beat::delete(&state, checked_beat_id).await;

    if let Err(err) = result {
        tracing::error!(?err, "Failed deleting beat for combo");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed deleting beat for combo",
        ))?;
    };

    Ok(())
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

impl CheckedComboId {
    async fn check_for_user(
        state: &AppState,
        user_id: UserId,
        combo_id: ComboId,
    ) -> Result<Self, (StatusCode, &'static str)> {
        let result = Combo::is_owned_by(state, combo_id, user_id).await;
        match result {
            Ok(true) => Ok(CheckedComboId::Owned(combo_id)),
            Ok(false) => Ok(CheckedComboId::NotFound),
            Err(err) => {
                tracing::error!(?err, "Failed looking up combo ownership");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed looking up combo ownership",
                ))
            }
        }
    }

    async fn check_anonymous(
        state: &AppState,
        combo_id: ComboId,
    ) -> Result<Self, (StatusCode, &'static str)> {
        let result = Combo::lookup(state, combo_id)
            .await
            .map_err(db_err_to_status)?;

        match result {
            Some(combo) if !combo.is_private => Ok(CheckedComboId::Public(combo.id)),
            Some(_combo) => Ok(CheckedComboId::NotFound),
            None => Ok(CheckedComboId::NotFound),
        }
    }

    async fn check_timestamp(
        &self,
        state: &AppState,
        timestamp_id: TimestampId,
    ) -> Result<CheckedTimestampId, (StatusCode, &'static str)> {
        let combo_id = match self {
            CheckedComboId::Owned(combo_id) | CheckedComboId::Public(combo_id) => combo_id,
            CheckedComboId::NotFound => return Ok(CheckedTimestampId::NotFound),
        };

        let result = Timestamp::combo_of_timestamp(state, timestamp_id)
            .await
            .map_err(db_err_to_status)?;
        match result {
            Some(c) => {
                if c.num() == combo_id.num() {
                    match self {
                        CheckedComboId::Owned(_) => Ok(CheckedTimestampId::Owned(timestamp_id)),
                        CheckedComboId::Public(_) => Ok(CheckedTimestampId::Public(timestamp_id)),
                        CheckedComboId::NotFound => unreachable!(),
                    }
                } else {
                    Ok(CheckedTimestampId::NotFound)
                }
            }
            None => Ok(CheckedTimestampId::NotFound),
        }
    }

    async fn check_beat(
        &self,
        state: &AppState,
        beat_id: BeatId,
    ) -> Result<CheckedBeatId, (StatusCode, &'static str)> {
        let combo_id = match self {
            CheckedComboId::Owned(combo_id) | CheckedComboId::Public(combo_id) => combo_id,
            CheckedComboId::NotFound => return Ok(CheckedBeatId::NotFound),
        };

        let result = Beat::combo_of_beat(state, beat_id)
            .await
            .map_err(db_err_to_status)?;
        match result {
            Some(c) => {
                if c.num() == combo_id.num() {
                    match self {
                        CheckedComboId::Owned(_) => Ok(CheckedBeatId::Owned(beat_id)),
                        CheckedComboId::Public(_) => Ok(CheckedBeatId::Public(beat_id)),
                        CheckedComboId::NotFound => unreachable!(),
                    }
                } else {
                    Ok(CheckedBeatId::NotFound)
                }
            }
            None => Ok(CheckedBeatId::NotFound),
        }
    }
}
