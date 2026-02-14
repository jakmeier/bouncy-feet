use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    combo::{Combo, ComboId},
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
