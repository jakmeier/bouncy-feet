use crate::layers::oidc::AdditionalClaims;
use crate::user::UserId;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use axum_oidc::OidcClaims;

#[derive(serde::Serialize)]
pub struct UserInfoResponse {
    id: i64,
    sub: Option<String>,
}

pub async fn user_info(
    claims: Option<OidcClaims<AdditionalClaims>>,
    Extension(user_id): Extension<UserId>,
) -> Response {
    let user_info = if let Some(claims) = &claims {
        UserInfoResponse {
            id: user_id.num(),
            sub: Some(claims.subject().to_string()),
        }
    } else {
        UserInfoResponse {
            id: user_id.num(),
            sub: None,
        }
    };
    (StatusCode::OK, Json(user_info)).into_response()
}
