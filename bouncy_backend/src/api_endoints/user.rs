use crate::peertube::user::PeerTubeHandle;
use crate::user::{User, UserId, UserSearchFilter};
use crate::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

#[derive(serde::Serialize)]
pub struct PrivateUserInfoResponse {
    /// BF API user id
    id: i64,
    /// Open ID connect ID
    ///
    /// Returns null for guest accounts, that have not been linked to a Keycloak user, yet.
    sub: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PublicUserInfoResponse {
    /// BF API user id
    pub id: i64,
    /// The user's PeerTube handle for their account, based on the info when
    /// they last logged in. It might have changed on PeerTube since then.
    pub peertube_handle: Option<PeerTubeHandle>,
    pub display_name: String,
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
    users: Vec<PublicUserInfoResponse>,
}

/// Retrieve information of the user that made the request.
///
/// Note: We must not use the OidcClaims here for retrieving the OIDC subject, as the user might be
/// logged in with guest credentials. Or maybe the login expired and claims are not available.
pub async fn user_info(Extension(user): Extension<User>) -> Response {
    let user_info: PrivateUserInfoResponse = PrivateUserInfoResponse {
        id: user.id.num(),
        sub: user.oidc_subject.map(|sub| sub.to_string()),
    };

    (StatusCode::OK, Json(user_info)).into_response()
}

/// Show publicly visible list of users, suitable for paginated lists.
pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<UserSearchParams>,
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

    let users = users
        .into_iter()
        .map(PublicUserInfoResponse::from)
        .collect();

    Json(UsersResponse { users }).into_response()
}

/// Show publicly visible list of a single user, suitable to display a profile page.
pub async fn user(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<UserId>,
) -> axum::response::Result<Json<PublicUserInfoResponse>, (StatusCode, &'static str)> {
    let result = User::lookup_public_info(&state, user_id).await;
    let Some(user) = result else {
        return Err((StatusCode::NOT_FOUND, "No such user"));
    };

    Ok(Json(user.into()))
}

impl From<crate::user::PublicUserData> for PublicUserInfoResponse {
    fn from(u: crate::user::PublicUserData) -> Self {
        PublicUserInfoResponse {
            id: u.id.num(),
            display_name: u.public_name,
            peertube_handle: u.peertube_handle,
        }
    }
}
