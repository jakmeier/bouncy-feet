use crate::api_endoints::combo::JsonResponse;
use crate::peertube::user::{PeerTubeAccount, PeerTubeHandle};
use crate::search::DbSearchCheckedString;
use crate::user::{TrustedUserSearchFilter, User, UserId};
use crate::{peertube, AppState};
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

#[derive(serde::Serialize, Debug)]
pub struct PublicUserInfoResponse {
    /// BF API user id
    pub id: i64,
    /// The user's PeerTube handle for their account, based on the info when
    /// they last logged in. It might have changed on PeerTube since then.
    pub peertube_handle: Option<PeerTubeHandle>,
    pub display_name: Option<String>,
    pub small_avatar: Option<String>,
}

#[derive(serde::Deserialize, Default)]
pub struct UserSearchParams {
    pub name_fragment: Option<String>,
    pub include_guests: Option<bool>,
    #[serde(default)]
    pub offset: i64,
    pub limit: Option<u32>,
}

#[derive(serde::Serialize, Debug)]
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
) -> JsonResponse<UsersResponse> {
    let name_fragment = if let Some(frag) = params.name_fragment {
        let checked = DbSearchCheckedString::check(frag).map_err(|err| {
            tracing::info!(?err, "invalid search request");
            (StatusCode::OK, "`name_fragment` invalid")
        })?;
        Some(checked)
    } else {
        None
    };
    let filter = TrustedUserSearchFilter {
        name_fragment,
        include_guests: params.include_guests.unwrap_or(false),
        offset: params.offset,
        limit: params.limit.unwrap_or(10).min(50) as u16,
    };

    let results = User::list(&state, &filter).await;
    let Ok(users) = results else {
        let err = results.unwrap_err();
        tracing::error!(?err, "Failed fetching users");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed fetching users"));
    };

    let mut users: Vec<_> = users
        .into_iter()
        .map(PublicUserInfoResponse::from_db_info)
        .collect();

    for user in &mut users {
        if let Some(handle) = &user.peertube_handle {
            let account = peertube::user::user_account(&state, handle).await;
            match &account {
                Ok(account_info) => user.add_peertube_info(account_info),
                Err(err) => tracing::warn!(?err, "Error reading user account info from PeerTube"),
            }
        }
    }

    Ok(Json(UsersResponse { users }))
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

    // Without small avatar here, assuming the client will look up the full
    // account info anyway.
    Ok(Json(PublicUserInfoResponse::from_db_info(user)))
}

impl PublicUserInfoResponse {
    /// Fills info available from the local DB but leaves fields that require a
    /// PeerTube API request empty.
    fn from_db_info(u: crate::user::PublicUserData) -> Self {
        PublicUserInfoResponse {
            id: u.id.num(),
            display_name: u.public_name,
            peertube_handle: u.peertube_handle,
            small_avatar: None,
        }
    }

    fn add_peertube_info(&mut self, account: &PeerTubeAccount) {
        if let Some(avatar) = account.avatars.first() {
            self.small_avatar = Some(avatar.file_url.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::test_helpers::{make_test_state, setup_deterministic_users};

    use super::*;

    /// Test setup and checker function for list_user test cases.
    ///
    /// Insert users to DB ahead of time, then call this.
    async fn check_list_user(
        state: &AppState,
        search: UserSearchParams,
        expect: expect_test::ExpectFile,
    ) {
        let result = list_users(State(state.clone()), Query(search)).await;
        expect.assert_debug_eq(&result);
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn test_list_users(pool: PgPool) {
        let state = &State(make_test_state(pool));
        let mut seed = 1000;
        seed = setup_deterministic_users(state, 3, 3, seed).await;

        check_list_user(
            state,
            UserSearchParams::default(),
            expect_test::expect_file!["./test_snapshots/list_users_1.snapshot"],
        )
        .await;

        for _ in 0..10 {
            seed = setup_deterministic_users(state, 10, 10, seed).await;
        }

        check_list_user(
            state,
            UserSearchParams::default(),
            expect_test::expect_file!["./test_snapshots/list_users_2.snapshot"],
        )
        .await;

        // with offset
        check_list_user(
            state,
            UserSearchParams {
                offset: 35,
                ..Default::default()
            },
            expect_test::expect_file!["./test_snapshots/list_users_3.snapshot"],
        )
        .await;

        // with guests included
        check_list_user(
            state,
            UserSearchParams {
                include_guests: Some(true),
                ..Default::default()
            },
            expect_test::expect_file!["./test_snapshots/list_users_4.snapshot"],
        )
        .await;

        // with search and guests included
        check_list_user(
            state,
            UserSearchParams {
                include_guests: Some(true),
                name_fragment: Some("13".to_owned()),
                ..Default::default()
            },
            expect_test::expect_file!["./test_snapshots/list_users_5.snapshot"],
        )
        .await;

        // with search and guests excluded
        check_list_user(
            state,
            UserSearchParams {
                include_guests: Some(false),
                name_fragment: Some("01".to_owned()),
                ..Default::default()
            },
            expect_test::expect_file!["./test_snapshots/list_users_6.snapshot"],
        )
        .await;
    }
}
