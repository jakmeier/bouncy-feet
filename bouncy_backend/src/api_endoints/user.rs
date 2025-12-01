use crate::user::{User, UserSearchFilter};
use crate::AppState;
use axum::extract::State;
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

#[derive(serde::Deserialize)]
pub struct UserSearchParams {
    // TODO: support searching
    // pub name_fragment: Option<String>,
    #[serde(default)]
    pub offset: i64,
}

#[derive(serde::Serialize)]
pub struct UsersResponse {
    names: Vec<String>,
}

/// Retrieve information of the user that made the request.
///
/// Note: We must not use the OidcClaims here for retrieving the OIDC subject, as the user might be
/// logged in with guest credentials. Or maybe the login expired and claims are not available.
pub async fn user_info(Extension(user): Extension<User>) -> Response {
    let user_info: UserInfoResponse = UserInfoResponse {
        id: user.id.num(),
        sub: user.oidc_subject.map(|sub| sub.to_string()),
    };

    (StatusCode::OK, Json(user_info)).into_response()
}

/// Show publicly visible list of users.
pub async fn list_users(
    State(state): State<AppState>,
    Json(params): Json<UserSearchParams>,
) -> Response {
    let filter = UserSearchFilter {
        include_guests: false,
        offset: params.offset,
        limit: 50,
    };
    let results = User::list(&state, &filter).await;
    let Ok(users) = results else {
        let err = results.unwrap_err();
        tracing::error!(?err, "Failed fetching users");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed fetching users").into_response();
    };

    // TODO: include display names, evaluate how user model should be on backend
    let ids = users.into_iter().map(|u| u.id.to_string()).collect();

    // TODO: better response type
    Json(UsersResponse { names: ids }).into_response()
}
