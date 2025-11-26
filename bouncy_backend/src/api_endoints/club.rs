use crate::db::club::Club;
use crate::user::User;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

#[derive(serde::Deserialize)]
pub struct CreateClubsRequest {
    title: String,
    description: String,
}

#[derive(serde::Serialize)]
struct ClubsResponse {
    clubs: Vec<ClubInfo>,
}

#[derive(serde::Serialize)]
struct ClubInfo {
    /// BF API club id = DB id
    id: i64,
    name: String,
    // lang: String, needed ?? -> not yet, maybe at some point
    description: String,
    public_playlist: String,
    // Only set for authenticated members
    private_playlist: Option<String>,
    // TODO
    // style

    // TODO: Option<privat + personal infos>
    // stats (#videos, # unseen videos, # members etc)
    // for channels, I could query only not seen videos
    // /api/v1/video-channels/{channelHandle}/videos
    // with excludeAlreadyWatched = true
    // But for playlists, this isn't currently available
    // /api/v1/video-playlists
    // does not have `excludeAlreadyWatched`
}

/// Retrieve clubs of the user that made the request.
pub async fn my_clubs(Extension(user): Extension<User>, State(state): State<AppState>) -> Response {
    let res = Club::list_clubs_for_user(&state, user.id).await;

    let Ok(db_clubs) = res else {
        let err = res.unwrap_err();
        tracing::error!(?err, "DB error on my_clubs");
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB ERROR").into_response();
    };

    let clubs = db_clubs_to_club_infos(db_clubs);
    let resonse = ClubsResponse { clubs };
    (StatusCode::OK, Json(resonse)).into_response()
}

fn db_clubs_to_club_infos(db_clubs: Vec<Club>) -> Vec<ClubInfo> {
    db_clubs
        .into_iter()
        .map(|club| ClubInfo {
            id: club.id.num(),
            name: club.title,
            description: club.description,
            public_playlist: club.public_playlist,
            // Listing own clubs -> access granted
            private_playlist: Some(club.private_playlist),
        })
        .collect()
}

/// Retrieve publicly listed clubs.
pub async fn clubs(State(state): State<AppState>) -> Response {
    // TODO: Add pagination parameters to API
    let res = Club::list(&state, 100, 0).await;

    let Ok(db_clubs) = res else {
        let err = res.unwrap_err();
        tracing::error!(?err, "DB error on clubs");
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB ERROR").into_response();
    };

    let clubs = db_clubs_to_club_infos(db_clubs);
    let resonse = ClubsResponse { clubs };
    (StatusCode::OK, Json(resonse)).into_response()
}

#[axum::debug_handler]
pub async fn create_club(
    State(state): State<AppState>,
    Json(payload): Json<CreateClubsRequest>,
) -> Response {
    if payload.title.len() > 64 {
        return (StatusCode::BAD_REQUEST, "Title must be at most 64 chars").into_response();
    }
    if payload.description.len() > 1024 {
        return (
            StatusCode::BAD_REQUEST,
            "Description must be at most 1024 chars",
        )
            .into_response();
    }
    // Check unique name? (not enforced on db right)
    // Limit clubs per user?

    // TODO: Generate playlists

    let public_playlist = "TODO".to_owned();
    let private_playlist = "TODO".to_owned();

    let res = Club::create(
        &state,
        &payload.title,
        &payload.description,
        &public_playlist,
        &private_playlist,
    )
    .await;

    if let Err(err) = res {
        tracing::error!(?err, "DB error on create_club");
        return (StatusCode::BAD_REQUEST, "Could not create club").into_response();
    }

    (StatusCode::CREATED, "CREATED").into_response()
}

// TODO
// #[axum(debug_handler)]
// pub async fn add_club_member(State(state): State<AppState>) -> Response {
//     // Check permissions: Must be admin
// }
