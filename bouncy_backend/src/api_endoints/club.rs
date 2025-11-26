use crate::db::club::Club;
use crate::peertube::playlist::{
    create_public_system_playlist, create_unlisted_system_playlist, Playlist,
};
use crate::peertube::{handle_peertube_error, PeerTubeError};
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
    let response = ClubsResponse { clubs };
    (StatusCode::OK, Json(response)).into_response()
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
    Extension(user): Extension<User>,
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
    // Check unique name? (not enforced on db)
    // Limit clubs per user?

    let mut max_retry = 5;
    let playlists = loop {
        let result = create_club_playlist_pair(&state, &payload.title).await;
        if let Err(e) = &result {
            let retry = handle_peertube_error(&state, e).await;
            if retry && max_retry > 0 {
                max_retry -= 1;
                continue;
            }
        }
        break result;
    };

    let Ok((public_playlist, private_playlist)) = playlists else {
        let err = playlists.unwrap_err();
        tracing::error!(?err, "failed creating playlists");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed creating playlists",
        )
            .into_response();
    };

    let res = Club::create(
        &state,
        &payload.title,
        &payload.description,
        &public_playlist.short_uuid,
        &private_playlist.short_uuid,
    )
    .await;

    if let Err(err) = res {
        // TODO: clean up inconsistent state (remove playlists)
        tracing::error!(?err, "DB error on create_club");
        return (StatusCode::BAD_REQUEST, "Could not create club").into_response();
    }

    // Add creator as first admin member
    let club = res.expect("just checked");
    let res_add_admin = Club::add_or_update_member(&state, user.id, club.id, true).await;
    if let Err(err) = res_add_admin {
        // TODO: clean up inconsistent state (remove playlists, delete club)
        tracing::error!(?err, "DB error on create_club -> add admin member");
        return (StatusCode::BAD_REQUEST, "Could not add as club admin").into_response();
    }

    let club_info = ClubInfo {
        id: club.id.num(),
        name: club.title,
        description: club.description,
        public_playlist: club.public_playlist,
        // Creator made the request -> access granted
        private_playlist: Some(club.private_playlist),
    };
    (StatusCode::CREATED, Json(club_info)).into_response()
}

async fn create_club_playlist_pair(
    state: &AppState,
    title: &str,
) -> Result<(Playlist, Playlist), PeerTubeError> {
    let channel_id = 1; // TODO: generate the admin channel, read it from AppState

    let public_display_name = format!("{title} - public videos");
    let public_description = format!("autogenerated public playlist for the club {title}");
    let private_display_name = format!("{title} - member-only videos");
    let private_description = format!("autogenerated unlisted playlist for the club {title}");

    let public_playlist =
        create_public_system_playlist(state, &public_display_name, &public_description, channel_id)
            .await?;
    let private_playlist = create_unlisted_system_playlist(
        state,
        &private_display_name,
        &private_description,
        channel_id,
    )
    .await?;

    Ok((public_playlist, private_playlist))
}

// TODO
// #[axum(debug_handler)]
// pub async fn add_club_member(State(state): State<AppState>) -> Response {
//     // Check permissions: Must be admin
// }
