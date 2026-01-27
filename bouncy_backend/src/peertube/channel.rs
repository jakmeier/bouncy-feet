use crate::{
    peertube::{check_peertube_response, check_peertube_system_user_response, PeerTubeError},
    AppState,
};
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub(crate) struct PeerTubeChannelId(pub i64);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub(crate) struct PeerTubeChannelHandle(pub String);

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct CreateChannel {
    name: String, // [ 1 .. 50 ] characters /^[a-zA-Z0-9\\-_.:]+$/
    #[serde(rename = "displayName")]
    display_name: String,
    description: Option<String>,
    // support: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct CreateChannelResponse {
    #[serde(rename = "videoChannel")]
    video_channel: ChannelIdResponse,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ChannelIdResponse {
    id: PeerTubeChannelId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelResponse {
    pub id: PeerTubeChannelId,
    pub url: Option<String>,
    pub name: String,
    pub avatars: Vec<Avatar>,
    pub host: String,
    pub host_redundancy_allowed: Option<bool>,
    pub following_count: i32,
    pub followers_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub display_name: String,
    pub description: Option<String>,
    pub support: Option<String>,
    pub is_local: bool,
    pub banners: Vec<Banner>,
    // always system channel, not interesting
    // pub owner_account: OwnerAccount,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub file_url: String,
    pub width: i32,
    pub height: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub file_url: String,
    pub width: i32,
    pub height: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) async fn create_system_channel(
    state: &AppState,
    name: String,
    display_name: String,
    description: Option<String>,
) -> Result<PeerTubeChannelId, PeerTubeError> {
    if !is_valid_name(&name, 1, 50) {
        return Err(PeerTubeError::ClientValidationError(name));
    }
    if display_name.len() > 255 {
        return Err(PeerTubeError::ClientValidationError(
            "display name too long".to_owned(),
        ));
    }

    let url = state
        .peertube_url
        .join("api/v1/video-channels")
        .expect("must be valid url");

    let token = state.system_user.access_token(state).await?;

    let body = CreateChannel {
        name,
        display_name,
        description,
    };

    let response = state
        .http_client
        .post(url.as_str())
        .bearer_auth(&token)
        .json(&body)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    let ok_response = check_peertube_system_user_response(response, token).await?;
    let status = ok_response.status();
    let channel: Result<CreateChannelResponse, _> = ok_response.json().await;
    let channel = channel.map_err(|err| PeerTubeError::JsonParsingFailed(status, err))?;
    Ok(channel.video_channel.id)
}

pub(crate) async fn delete_system_channel(
    state: &AppState,
    channel_handle: &PeerTubeChannelHandle,
) -> Result<(), PeerTubeError> {
    let url = state
        .peertube_url
        .join(&format!("api/v1/video-channels/{}", channel_handle.0))
        .expect("must be valid url");

    let token = state.system_user.access_token(state).await?;

    let response = state
        .http_client
        .delete(url.as_str())
        .bearer_auth(&token)
        .send()
        .await;

    let _ok_response = check_peertube_system_user_response(response, token).await?;
    Ok(())
}

pub async fn fetch_channel(
    state: &AppState,
    channel: &PeerTubeChannelHandle,
) -> Result<ChannelResponse, PeerTubeError> {
    let url = state
        .peertube_url
        .join(&format!("api/v1/video-channels/{}", channel.0))
        .expect("must be valid url");

    let response = state.http_client.get(url).send().await;
    let ok_response = check_peertube_response(response).await?;
    let status = ok_response.status();
    let channel: Result<ChannelResponse, _> = ok_response.json().await;
    let channel = channel.map_err(|err| PeerTubeError::JsonParsingFailed(status, err))?;
    Ok(channel)
}

pub async fn update_avatar(
    state: &AppState,
    channel: PeerTubeChannelHandle,
    file_bytes: Vec<u8>,
) -> Result<(), PeerTubeError> {
    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name("avatar.png".to_string())
        .mime_str("image/png")
        .unwrap();

    let form = reqwest::multipart::Form::new().part("avatarfile", part);

    let url = state
        .peertube_url
        .join(&format!("api/v1/video-channels/{}/avatar/pick", channel.0))
        .expect("must be valid url");

    let token = state.system_user.access_token(state).await?;

    let response = state
        .http_client
        .post(url)
        .bearer_auth(&token)
        .multipart(form)
        .send()
        .await;

    let _ok_response = check_peertube_system_user_response(response, token).await?;

    Ok(())
}

/// [ min .. max ] characters /^[a-zA-Z0-9\\-_.:]+$/
fn is_valid_name(s: &str, min: usize, max: usize) -> bool {
    s.len() >= min
        && s.len() <= max
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':'))
}

impl PeerTubeChannelId {
    pub fn num(&self) -> i64 {
        self.0
    }
}
