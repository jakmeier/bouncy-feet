use crate::user::User;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

#[derive(serde::Serialize)]
pub struct UserInfoResponse {
    /// BF API user id
    id: i64,
    /// Open ID connect ID
    ///
    /// Returns null for guest accounts, that have not been linked to a Keycloak user, yet.
    sub: Option<String>,
}

/// Retrieve information of the user that made the request.
///
/// Note: We must not use the OidcClaims here for retrieving the OIDC subject, as the user might be
/// logged in with guest credentials. Or maybe the login expired and claims are not available.
pub async fn user_info(Extension(user): Extension<User>) -> Response {
    let user_info = UserInfoResponse {
        id: user.id.num(),
        sub: user.oidc_subject.map(|sub| sub.to_string()),
    };

    (StatusCode::OK, Json(user_info)).into_response()
}
