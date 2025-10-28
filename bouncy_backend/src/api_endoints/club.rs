use crate::user::User;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

#[derive(serde::Serialize)]
struct ClubsResponse {
    clubs: Vec<ClubInfo>,
}

#[derive(serde::Serialize)]
struct ClubInfo {
    /// BF API club id = DB id
    id: i64,
    name: String,
    // lang: String, needed ??
    description: String,
    // TODO
    // style
    // peertubePlaylist

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
pub async fn my_clubs(Extension(_user): Extension<User>) -> Response {
    // TODO: read from DB, this is just a mock
    let resonse = ClubsResponse {
        clubs: vec![ClubInfo {
            id: 1,
            name: "East Attitude Shufflers".to_owned(),
            // lang: "de".to_owned(),
            description: "Weekly Shuffle Trainings Gruppe".to_owned(),
        }],
    };

    (StatusCode::OK, Json(resonse)).into_response()
}

/// Retrieve publicly listed clubs.
pub async fn clubs() -> Response {
    let resonse = ClubsResponse { clubs: vec![] };
    (StatusCode::OK, Json(resonse)).into_response()
}
