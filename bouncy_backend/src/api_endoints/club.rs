use crate::api_endoints::user::PublicUserInfoResponse;
use crate::club::{ClubId, ClubMembership, PlaylistInfo, PublicClubMemberInfo};
use crate::db::club::Club;
use crate::peertube::channel::{create_system_channel, ChannelId};
use crate::peertube::playlist::{
    create_public_system_playlist, create_unlisted_system_playlist, Playlist,
};
use crate::peertube::{handle_peertube_error, PeerTubeError};
use crate::user::{User, UserId};
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

/// Club summary, contains information for displaying a list of clubs.
#[derive(serde::Serialize)]
struct ClubInfo {
    /// BF API club id = DB id
    id: i64,
    name: String,
    // lang: String, needed ?? -> not yet, maybe at some point
    description: String,
    public_playlist: PlaylistInfo,
    // Only set for authenticated members
    private_playlist: Option<PlaylistInfo>,
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

/// Additional details about a club, retrieved by /clubs/{club_id}.
///
/// Conatains information to show a detailed club view.
#[derive(serde::Serialize)]
pub(crate) struct ClubDetails {
    admins: Vec<PublicUserInfoResponse>,
    members: Vec<PublicUserInfoResponse>,
}

#[derive(serde::Deserialize)]
pub struct AddClubMemberRequest {
    // Backend / DB user id
    pub user_id: i64,
    pub club_id: i64,
}

#[derive(serde::Deserialize)]
pub struct AddClubVideoRequest {
    pub video_id: i64,
    pub club_id: i64,
    pub private: bool,
}

/// Retrieve clubs of the user that made the request.
pub async fn my_clubs(Extension(user): Extension<User>, State(state): State<AppState>) -> Response {
    let res = Club::list_clubs_for_user(&state, user.id).await;

    let Ok(db_clubs) = res else {
        let err = res.unwrap_err();
        tracing::error!(?err, "DB error on my_clubs");
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB ERROR").into_response();
    };

    // Listing own clubs -> access granted
    let private_access = true;
    let clubs = db_clubs_to_club_infos(db_clubs, private_access);
    let response = ClubsResponse { clubs };
    (StatusCode::OK, Json(response)).into_response()
}

fn db_clubs_to_club_infos(db_clubs: Vec<Club>, has_private_access: bool) -> Vec<ClubInfo> {
    db_clubs
        .into_iter()
        .map(|club| ClubInfo {
            id: club.id.num(),
            name: club.title,
            description: club.description,
            public_playlist: club.public_playlist,
            private_playlist: has_private_access.then_some(club.private_playlist),
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

    let private_access = false;
    let clubs = db_clubs_to_club_infos(db_clubs, private_access);
    let resonse = ClubsResponse { clubs };
    (StatusCode::OK, Json(resonse)).into_response()
}

/// Retrieve extended info about a club, public info only.
#[axum::debug_handler]
pub async fn club(
    State(state): State<AppState>,
    axum::extract::Path(club_id): axum::extract::Path<ClubId>,
) -> axum::response::Result<Json<ClubDetails>> {
    let res = Club::list_members_with_info(&state, club_id, 100, 0).await;

    let Ok(mut db_members) = res else {
        let err = res.unwrap_err();
        tracing::error!(?err, "DB error on listing club members");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "DB ERROR"))?;
    };

    fn row_to_user_info(row: PublicClubMemberInfo) -> PublicUserInfoResponse {
        PublicUserInfoResponse {
            id: row.user_id.num(),
            display_name: row.public_name,
        }
    }

    let admins = db_members
        .extract_if(.., |row| matches!(row.membership, ClubMembership::Admin))
        .map(row_to_user_info)
        .collect();
    let members = db_members.into_iter().map(row_to_user_info).collect();

    let details = ClubDetails { admins, members };
    Ok(Json(details))
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
    let channel_result = loop {
        let result = create_club_channel(&state, &payload.title).await;
        if let Err(e) = &result {
            let retry = handle_peertube_error(&state, e).await;
            if retry && max_retry > 0 {
                max_retry -= 1;
                continue;
            }
        }
        break result;
    };
    let Ok(channel_id) = channel_result else {
        let err = channel_result.unwrap_err();
        tracing::error!(?err, "failed creating channel");
        return (StatusCode::INTERNAL_SERVER_ERROR, "failed creating channel").into_response();
    };

    let mut max_retry = 5;
    let playlists = loop {
        let result = create_club_playlist_pair(&state, &payload.title, channel_id).await;
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
        public_playlist.id,
        &public_playlist.short_uuid,
        private_playlist.id,
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

async fn create_club_channel(state: &AppState, name: &str) -> Result<ChannelId, PeerTubeError> {
    let description = format!("auto-generated channel for the club {name}");
    let display_name = name.to_owned();
    let Some(id_name) = display_name_to_username(&display_name) else {
        return Err(PeerTubeError::ClientValidationError(display_name));
    };

    create_system_channel(state, id_name, display_name, Some(description)).await
}

async fn create_club_playlist_pair(
    state: &AppState,
    title: &str,
    channel_id: ChannelId,
) -> Result<(Playlist, Playlist), PeerTubeError> {
    let public_display_name = format!("{title} - public videos");
    let public_description = format!("auto-generated public playlist for the club {title}");
    let private_display_name = format!("{title} - member-only videos");
    let private_description = format!("auto-generated unlisted playlist for the club {title}");

    let public_playlist = create_public_system_playlist(
        state,
        &public_display_name,
        &public_description,
        channel_id.num(),
    )
    .await?;
    let private_playlist = create_unlisted_system_playlist(
        state,
        &private_display_name,
        &private_description,
        channel_id.num(),
    )
    .await?;

    Ok((public_playlist, private_playlist))
}

#[axum::debug_handler]
pub async fn add_club_member(
    Extension(me): Extension<User>,
    State(state): State<AppState>,
    Json(params): Json<AddClubMemberRequest>,
) -> Response {
    // Check permissions: Must be admin
    let membership = match get_membership(&state, me.id, params.club_id()).await {
        Ok(it) => it,
        Err(response) => return response,
    };
    if !matches!(membership, ClubMembership::Admin) {
        return (StatusCode::FORBIDDEN, "no permission to add club member").into_response();
    }

    // Check user exist to avoid accidentally overriding is_admin
    let result = Club::membership(&state, params.user_id(), params.club_id()).await;
    let their_membership = result.expect("just checked");
    if !matches!(their_membership, ClubMembership::None) {
        return (StatusCode::OK, "already a member").into_response();
    }

    let is_admin = false;
    let result =
        Club::add_or_update_member(&state, params.user_id(), params.club_id(), is_admin).await;
    if let Err(err) = result {
        tracing::error!(?err, "DB error on add_club_member -> add_or_update_member");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }
    (StatusCode::CREATED, "member added").into_response()
}

#[axum::debug_handler]
pub async fn add_video(
    Extension(me): Extension<User>,
    State(state): State<AppState>,
    Json(params): Json<AddClubVideoRequest>,
) -> Response {
    // Check permissions: Must be member
    // Side-note: maybe should be admin for public videos?
    let membership = match get_membership(&state, me.id, params.club_id()).await {
        Ok(it) => it,
        Err(response) => return response,
    };
    if !matches!(membership, ClubMembership::Admin | ClubMembership::Member) {
        return (StatusCode::FORBIDDEN, "not a club member").into_response();
    }

    // Check the playlist that belongs to the club.
    // The user would know it and could provide it but we need to check for permissions anyway.
    let Some(club) = Club::lookup(&state, params.club_id()).await else {
        return (StatusCode::NOT_FOUND, "no such club").into_response();
    };

    let playlist = if params.private {
        club.private_playlist
    } else {
        club.public_playlist
    };

    let result =
        crate::peertube::playlist::add_video_to_playlist(&state, params.video_id, playlist.id)
            .await;

    if let Err(err) = result {
        tracing::error!(?err, "Adding video to playlist failed");
        return (
            StatusCode::BAD_GATEWAY,
            "Could not add video to PeerTube playlist",
        )
            .into_response();
    }
    let element = result.expect("just checked");

    (StatusCode::CREATED, element.id.to_string()).into_response()
}

async fn get_membership(
    state: &AppState,
    user_id: UserId,
    club_id: ClubId,
) -> Result<ClubMembership, Response> {
    let result = Club::membership(state, user_id, club_id).await;
    if let Err(err) = result {
        tracing::error!(?err, "DB error on reading club membership");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response());
    }

    Ok(result.expect("just checked"))
}

fn display_name_to_username(input: &str) -> Option<String> {
    let mut out = String::with_capacity(input.len());
    // no starting dash
    let mut prev_dash = true;

    for c in input.trim().chars() {
        let c = c.to_ascii_lowercase();

        match c {
            // PeerTube also allows ':' but we don't want it.
            'a'..='z' | '0'..='9' | '.' | '_' => {
                out.push(c);
                prev_dash = false;
            }

            // handle a few special cases
            // expand as needed (or use deunicode or something)
            'ö' | 'Ö' => {
                out.push('o');
                out.push('e');
            }
            'ä' | 'Ä' => {
                out.push('a');
                out.push('e');
            }
            'ü' | 'Ü' => {
                out.push('u');
                out.push('e');
            }
            'ß' => {
                out.push('s');
                out.push('s');
            }
            'é' | 'è' | 'ê' | 'É' => out.push('e'),
            'à' | 'á' | 'â' | 'Á' => out.push('a'),
            'ï' | 'í' | 'Í' => out.push('i'),
            'ó' | 'Ó' => out.push('o'),
            'ú' | 'Ú' => out.push('u'),

            // ñ
            'ñ' | 'Ñ' => out.push('n'),
            _ => {
                if !prev_dash {
                    out.push('-');
                    prev_dash = true;
                }
            }
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_name_to_username() {
        fn check_ok(input: &str, expect: &str) {
            let output = display_name_to_username(input).unwrap();
            assert_eq!(output, expect);
        }
        fn check_err(input: &str) {
            let output = display_name_to_username(input);
            assert_eq!(output, None);
        }

        check_ok("Hello World", "hello-world");
        check_ok("CoolClub", "coolclub");
        check_ok("Bouncy Feet! (teachers)", "bouncy-feet-teachers");
        check_ok(
            "Die Größten Kinder: Bla Bla",
            "die-groessten-kinder-bla-bla",
        );
        check_ok("DE ÖöÄäÜüß", "de-oeoeaeaeueuess");
        check_ok("FR éèêàáâ ", "fr-eeeaaa");
        check_ok("EN ï ", "en-i");
        check_ok("ES ÁáÉéÍíÓóÚúÑñ¿¡", "es-aaeeiioouunn");

        check_err("");
        check_err("-");
        check_err("----");
        check_err("*");
        check_err(" - ");
    }
}
