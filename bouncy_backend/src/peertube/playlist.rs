use crate::{
    peertube::{check_peertube_system_user_response, PeerTubeError},
    AppState,
};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct Playlist {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub uuid: Uuid,
    #[serde(alias = "shortUUID")]
    pub short_uuid: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct PlaylistCreatedResponse {
    #[serde(alias = "videoPlaylist")]
    video_playlist: Playlist,
}

#[derive(Clone, Copy, Debug)]
enum PlaylistPrivacy {
    Public = 1,
    Unlisted = 2,
    #[allow(dead_code)]
    Private = 3,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct VideoAddedResponse {
    #[serde(alias = "videoPlaylistElement")]
    video_playlist_element: PlaylistElement,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct PlaylistElement {
    pub id: i64,
}

/// Creates an impersonal (system) playlist on PeerTube that's unlisted.
pub(crate) async fn create_unlisted_system_playlist(
    state: &AppState,
    display_name: &str,
    description: &str,
    channel_id: i64,
) -> Result<Playlist, PeerTubeError> {
    let body = [
        ("displayName", display_name),
        ("description", description),
        ("privacy", PlaylistPrivacy::Unlisted.to_num_str()),
        ("videoChannelId", &channel_id.to_string()),
    ];
    create_system_playlist(state, &body).await
}

/// Creates an impersonal (system) playlist on PeerTube that's publicly listed.
pub(crate) async fn create_public_system_playlist(
    state: &AppState,
    display_name: &str,
    description: &str,
    channel_id: i64,
) -> Result<Playlist, PeerTubeError> {
    let body = [
        ("displayName", display_name),
        ("description", description),
        ("privacy", PlaylistPrivacy::Public.to_num_str()),
        ("videoChannelId", &channel_id.to_string()),
    ];
    create_system_playlist(state, &body).await
}

async fn create_system_playlist<T: serde::Serialize + ?Sized>(
    state: &AppState,
    body: &T,
) -> Result<Playlist, PeerTubeError> {
    let url = state
        .peertube_url
        .join("api/v1/video-playlists")
        .expect("must be valid url");

    let token = state.system_user.access_token(state).await?;

    let response = state
        .http_client
        .post(url.as_str())
        .form(body)
        .bearer_auth(&token)
        .send()
        .await;

    let ok_response = check_peertube_system_user_response(response, token).await?;
    let status = ok_response.status();
    let playlist: Result<PlaylistCreatedResponse, _> = ok_response.json().await;
    let playlist = playlist.map_err(|err| PeerTubeError::JsonParsingFailed(status, err))?;
    Ok(playlist.video_playlist)
}

pub(crate) async fn add_video_to_playlist(
    state: &AppState,
    video_id: i64,
    playlist_id: i64,
) -> Result<PlaylistElement, PeerTubeError> {
    let url = state
        .peertube_url
        .join(&format!("api/v1/video-playlists/{}/videos", playlist_id))
        .expect("must be valid url");

    let token = state.system_user.access_token(state).await?;

    let response = state
        .http_client
        .post(url.as_str())
        .form(&[("videoId", video_id)])
        .bearer_auth(&token)
        .send()
        .await;

    let ok_response = check_peertube_system_user_response(response, token).await?;
    let status = ok_response.status();
    let msg: Result<VideoAddedResponse, _> = ok_response.json().await;
    let msg = msg.map_err(|err| PeerTubeError::JsonParsingFailed(status, err))?;
    Ok(msg.video_playlist_element)
}

impl PlaylistPrivacy {
    #[inline(always)]
    fn to_num_str(self) -> &'static str {
        match self as usize {
            1 => "1",
            2 => "2",
            3 => "3",
            _ => unreachable!("should not have other variants"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist_privacy_enum_to_num_str() {
        check_enum_to_num_str(PlaylistPrivacy::Private);
        check_enum_to_num_str(PlaylistPrivacy::Public);
        check_enum_to_num_str(PlaylistPrivacy::Unlisted);
    }

    #[track_caller]
    fn check_enum_to_num_str(variant: PlaylistPrivacy) {
        assert_eq!((variant as usize).to_string(), variant.to_num_str());
    }
}
