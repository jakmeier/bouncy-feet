use crate::api_endoints::user::PublicUserInfoResponse;
use crate::club::{ClubId, ClubMembership, PublicClubMemberInfo};
use crate::db::club::Club;
use crate::peertube::channel::{create_system_channel, PeerTubeChannelHandle, PeerTubeChannelId};
use crate::peertube::playlist::{create_public_system_playlist, create_unlisted_system_playlist};
use crate::peertube::{self, handle_peertube_error, PeerTubeError};
use crate::playlist::{Playlist, PlaylistInfo};
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
    url: Option<String>,
}

#[derive(serde::Serialize)]
struct ClubsResponse {
    clubs: Vec<ClubInfo>,
}

#[derive(serde::Deserialize)]
pub struct UpdateClubRequest {
    description: String,
    url: Option<String>,
}

/// Club summary, contains information for displaying a list of clubs.
#[derive(serde::Serialize)]
struct ClubInfo {
    /// BF API club id = DB id
    id: i64,
    name: String,
    // lang: String, needed ?? -> not yet, maybe at some point
    description: String,

    /// full file path for 120x120 pixel avatar of the channel
    avatar: Option<String>,
    // could be added but not needed for listing
    // channel_id: Option<PeerTubeChannelId>,
    // channel_handle: Option<PeerTubeChannelHandle>,

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
/// Contains information to show a detailed club view.
#[derive(serde::Serialize)]
pub(crate) struct ClubDetails {
    // public
    admins: Vec<PublicUserInfoResponse>,
    num_members: u32,
    channel_id: Option<PeerTubeChannelId>,
    channel_handle: Option<PeerTubeChannelHandle>,
    main_playlist: Option<PlaylistInfo>,
    public_playlists: Vec<PlaylistInfo>,
    web_link: Option<String>,
    // only visible for members
    private: Option<PrivateClubDetails>,
}

#[derive(serde::Serialize)]
pub(crate) struct PrivateClubDetails {
    /// Club members, without admins
    members: Vec<PublicUserInfoResponse>,
    private_playlists: Vec<PlaylistInfo>,
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

    let clubs = db_clubs_to_club_infos(&state, db_clubs).await;
    let response = ClubsResponse { clubs };
    (StatusCode::OK, Json(response)).into_response()
}

async fn db_clubs_to_club_infos(state: &AppState, db_clubs: Vec<Club>) -> Vec<ClubInfo> {
    let mut out = Vec::with_capacity(db_clubs.len());
    for club in db_clubs {
        let mut avatar = None;
        if let Some(channel_handle) = &club.channel_handle {
            // TODO: these requests require caching
            let response = peertube::channel::fetch_channel(state, channel_handle).await;

            if let Ok(channel) = response {
                if !channel.avatars.is_empty() {
                    let preferred_size = channel
                        .avatars
                        .iter()
                        .find(|a| a.height >= 120)
                        .unwrap_or(&channel.avatars[0]);
                    avatar = Some(preferred_size.file_url.clone());
                }
            } else {
                let err = response.unwrap_err();
                tracing::warn!(?err, "Failed reading channel");
            }
        }
        let info = ClubInfo {
            id: club.id.num(),
            name: club.title,
            description: club.description,
            avatar,
        };
        out.push(info)
    }
    out
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

    let clubs = db_clubs_to_club_infos(&state, db_clubs).await;
    let response = ClubsResponse { clubs };
    (StatusCode::OK, Json(response)).into_response()
}

/// Retrieve extended info about a club, public info only.
#[axum::debug_handler]
pub async fn club(
    State(state): State<AppState>,
    axum::extract::Path(club_id): axum::extract::Path<ClubId>,
) -> axum::response::Result<Json<ClubDetails>> {
    club_details(&state, club_id, false).await
}

/// Retrieve extended info about a club, with private info.
#[axum::debug_handler]
pub async fn club_with_private_details(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    axum::extract::Path(club_id): axum::extract::Path<ClubId>,
) -> axum::response::Result<Json<ClubDetails>> {
    let membership = get_membership(&state, user.id, club_id).await?;
    if matches!(membership, ClubMembership::None) {
        return Err((StatusCode::FORBIDDEN, "must be member of club").into());
    }

    club_details(&state, club_id, true).await
}

pub async fn club_details(
    state: &AppState,
    club_id: ClubId,
    is_member: bool,
) -> axum::response::Result<Json<ClubDetails>> {
    let res = Club::list_members_with_info(state, club_id, 100, 0).await;

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
    let members: Vec<_> = db_members.into_iter().map(row_to_user_info).collect();

    let Some(club) = Club::lookup(state, club_id).await else {
        return Err((StatusCode::NOT_FOUND, "no such club"))?;
    };

    let mut main_playlist = None;
    if let Some(main_playlist_id) = club.main_playlist {
        let Some(playlist) = Playlist::lookup_club_playlist(state, main_playlist_id).await else {
            return Err((StatusCode::NOT_FOUND, "no such playlist").into_response())?;
        };
        main_playlist = Some(playlist);
    }

    let mut playlists = Playlist::lookup_club_playlists(state, club.id).await;
    let private_playlists: Vec<PlaylistInfo> = playlists
        .extract_if(.., |p| p.is_private)
        .map(Playlist::playlist_info)
        .collect();
    let public_playlists = playlists.into_iter().map(Playlist::playlist_info).collect();
    let num_members = u32::try_from(members.len()).unwrap_or(u32::MAX);

    let private = if is_member {
        Some(PrivateClubDetails {
            members,
            private_playlists,
        })
    } else {
        None
    };

    let details = ClubDetails {
        admins,
        num_members,
        main_playlist: main_playlist.map(Playlist::playlist_info),
        public_playlists,
        channel_id: club.channel_id,
        channel_handle: club.channel_handle,
        web_link: club.web_link,
        private,
    };
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
    let maybe_url = match validate_web_link(payload.url.as_deref()) {
        Ok(it) => it,
        Err(err) => return err.into_response(),
    };

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
    let Ok((channel_id, channel_handle)) = channel_result else {
        let err = channel_result.unwrap_err();
        tracing::error!(?err, "failed creating channel");
        return (StatusCode::INTERNAL_SERVER_ERROR, "failed creating channel").into_response();
    };

    let club_res: Result<Club, sqlx::Error> = Club::create(
        &state,
        &payload.title,
        &payload.description,
        maybe_url,
        channel_id,
        channel_handle,
        // Create without playlist first, so the playlist can be created with a club id.
        None,
    )
    .await;

    let Ok(club) = club_res else {
        // TODO: clean up inconsistent state (delete playlists on PeerTube)
        let err = club_res.unwrap_err();
        tracing::error!(?err, "DB error on create_club");
        return (StatusCode::BAD_REQUEST, "Could not create club").into_response();
    };

    let mut max_retry = 5;
    let playlists = loop {
        let result = create_club_playlist_pair(&state, &payload.title, channel_id, club.id).await;
        if let Err(e) = &result {
            let retry = handle_peertube_error(&state, e).await;
            if retry && max_retry > 0 {
                max_retry -= 1;
                continue;
            }
        }
        break result;
    };

    // TODO: do I even need a private playlist at this point?
    let Ok((public_playlist, _private_playlist)) = playlists else {
        let err = playlists.unwrap_err();
        tracing::error!(?err, "failed creating playlists");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed creating playlists",
        )
            .into_response();
    };

    // Now set the main playlist
    let res = Club::set_main_playlist(&state, club.id, public_playlist.id).await;
    if let Err(err) = res {
        tracing::error!(?err, "failed setting main playlist");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed setting club main playlist",
        )
            .into_response();
    }

    // Add creator as first admin member
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
        // The avatar isn't set on the channel, yet. Even if the user selected
        // one, is's set with an update afterwards. Not ideal, but it can be
        // handled by the client to show display it after the update.
        avatar: None,
    };
    (StatusCode::CREATED, Json(club_info)).into_response()
}

async fn create_club_channel(
    state: &AppState,
    name: &str,
) -> Result<(PeerTubeChannelId, PeerTubeChannelHandle), PeerTubeError> {
    let description = format!("auto-generated channel for the club {name}");
    let display_name = name.to_owned();
    let Some(id_name) = display_name_to_username(&display_name) else {
        return Err(PeerTubeError::ClientValidationError(display_name));
    };

    let id = create_system_channel(state, id_name.clone(), display_name, Some(description)).await?;
    Ok((id, PeerTubeChannelHandle(id_name)))
}

async fn create_club_playlist_pair(
    state: &AppState,
    title: &str,
    channel_id: PeerTubeChannelId,
    club_id: ClubId,
) -> Result<(Playlist, Playlist), PeerTubeError> {
    let public_display_name = format!("{title} - public videos");
    let public_description = format!("auto-generated public playlist for the club {title}");
    let private_display_name = format!("{title} - member-only videos");
    let private_description = format!("auto-generated unlisted playlist for the club {title}");

    // create externally
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

    // insert to DB
    let private = Playlist::create(state, club_id, private_playlist, true)
        .await
        .expect("creating playlist failed");
    let public = Playlist::create(state, club_id, public_playlist, false)
        .await
        .expect("creating playlist failed");

    Ok((public, private))
}

#[axum::debug_handler]
pub async fn update_club(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    axum::extract::Path(club_id): axum::extract::Path<ClubId>,
    Json(payload): Json<UpdateClubRequest>,
) -> Response {
    if payload.description.len() > 1024 {
        return (
            StatusCode::BAD_REQUEST,
            "Description must be at most 1024 chars",
        )
            .into_response();
    }
    if payload.url.as_ref().is_some_and(|s| s.len() > 255) {
        return (StatusCode::BAD_REQUEST, "URL must be at most 255 chars").into_response();
    }
    let maybe_url = match validate_web_link(payload.url.as_deref()) {
        Ok(it) => it,
        Err(err) => return err.into_response(),
    };

    // Check user has permission
    let membership = match get_membership(&state, user.id, club_id).await {
        Ok(it) => it,
        Err(response) => return response,
    };
    if !matches!(membership, ClubMembership::Admin) {
        return (StatusCode::FORBIDDEN, "no permission to update club").into_response();
    }

    let res = Club::set_meta_fields(&state, club_id, payload.description, maybe_url).await;
    if let Err(err) = res {
        tracing::error!(?err, "DB error on update_club");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    (StatusCode::OK, "OK").into_response()
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
        // TODO: API for uploading to a specific playlist
        return (
            StatusCode::BAD_REQUEST,
            "cannot add private video through this API",
        )
            .into_response();
    } else {
        club.main_playlist
    };

    let Some(main_playlist_id) = playlist else {
        tracing::error!(
            ?club,
            "User tried to upload to a club without a main playlist."
        );
        return (StatusCode::NOT_FOUND, "no main playlist").into_response();
    };

    let Some(playlist) = Playlist::lookup_club_playlist(&state, main_playlist_id).await else {
        return (StatusCode::NOT_FOUND, "no such playlist").into_response();
    };

    let result = crate::peertube::playlist::add_video_to_playlist(
        &state,
        params.video_id,
        playlist.peertube_info.id,
    )
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

#[axum::debug_handler]
pub async fn update_avatar(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    axum::extract::Path(club_id): axum::extract::Path<ClubId>,
    mut multipart: axum::extract::Multipart,
) -> Response {
    // Check user has permission
    let membership = match get_membership(&state, user.id, club_id).await {
        Ok(it) => it,
        Err(response) => return response,
    };
    if !matches!(membership, ClubMembership::Admin) {
        return (StatusCode::FORBIDDEN, "no permission to update club avatar").into_response();
    }

    let mut file_bytes = None;

    // Read multipart fields
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("");

        if name == "avatar" {
            let content_type = field.content_type().map(|s| s.to_string());

            if content_type.as_deref() != Some("image/png") {
                return (StatusCode::BAD_REQUEST, "image must be png").into_response();
            }

            let data = field.bytes().await.unwrap();
            file_bytes = Some(data);
            break;
        }
    }

    let file_bytes = match file_bytes {
        Some(it) => it,
        None => return (StatusCode::BAD_REQUEST, "No image uploaded").into_response(),
    };

    // find club channel handle
    let Some(club) = Club::lookup(&state, club_id).await else {
        return (StatusCode::NOT_FOUND, "no such club").into_response();
    };
    let Some(channel_handle) = club.channel_handle else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "club has no channel").into_response();
    };

    // upload image to PeerTube
    let mut max_retry = 5;
    let final_result = loop {
        let result =
            peertube::channel::update_avatar(&state, channel_handle.clone(), file_bytes.to_vec())
                .await;
        if let Err(e) = &result {
            let retry = handle_peertube_error(&state, e).await;
            if retry && max_retry > 0 {
                max_retry -= 1;
                continue;
            }
        }
        break result;
    };
    if let Err(err) = final_result {
        tracing::error!(?err, "failed uploading avatar");
        return (StatusCode::BAD_GATEWAY, "failed uploading avatar").into_response();
    };

    (StatusCode::OK, "OK").into_response()
}

fn validate_web_link(
    raw_url: Option<&str>,
) -> Result<Option<url::Url>, (StatusCode, &'static str)> {
    if raw_url.as_ref().is_some_and(|s| s.len() > 255) {
        return Err((StatusCode::BAD_REQUEST, "URL must be at most 255 chars"));
    }

    if let Some(raw_url) = &raw_url {
        let url = url::Url::parse(raw_url);
        match url {
            Ok(it) => Ok(Some(it)),
            Err(_err) => Err((StatusCode::BAD_REQUEST, "invalid URL in web_link")),
        }
    } else {
        Ok(None)
    }
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
