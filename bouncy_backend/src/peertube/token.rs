use crate::api_endoints::peertube_token::{OAuthClientConfig, TokenExchangeResponse};
use crate::peertube::{check_peertube_response, PeerTubeError};
use crate::AppState;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct OAuthToken {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: Option<String>,
}

impl OAuthToken {
    pub fn bearer_string(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}

pub(crate) async fn fetch_api_token_from_bypass_token(
    state: &AppState,
    bypass_token: TokenExchangeResponse,
) -> Result<OAuthToken, PeerTubeError> {
    fetch_api_token(
        state,
        bypass_token.username.as_str(),
        "externalAuthToken",
        bypass_token.externalAuthToken.as_str(),
    )
    .await
}

/// For local PeerTube users.
///
/// This should only be used for system users. Real users have an external
/// account managed through Keycloak.
pub(crate) async fn fetch_api_token_from_native_user(
    state: &AppState,
    user_name: &str,
    password: &str,
) -> Result<OAuthToken, PeerTubeError> {
    fetch_api_token(state, user_name, "password", password).await
}

async fn fetch_api_token(
    state: &AppState,
    username: &str,
    auth_method: &str,
    auth_secret: &str,
) -> Result<OAuthToken, PeerTubeError> {
    // Fetch an OAuth token for the user that's usable by the API
    async fn send_token_request(
        state: &AppState,
        client_config: OAuthClientConfig,
        username: &str,
        auth_method: &str,
        auth_secret: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let token_url = state
            .peertube_url
            .join("api/v1/users/token")
            .expect("invalid base url");
        state
            .http_client
            .post(token_url.as_str())
            .form(&[
                ("username", username),
                (auth_method, auth_secret),
                ("client_id", client_config.client_id.as_str()),
                ("client_secret", client_config.client_secret.as_str()),
                ("grant_type", "password"),
            ])
            .send()
            .await
    }

    let client_config = read_or_fetch_client_config(state)
        .await
        .map_err(PeerTubeError::ClientError)?;
    let token_res =
        send_token_request(state, client_config, username, auth_method, auth_secret).await;

    let ok_res = match check_peertube_response(token_res).await {
        Ok(ok_result) => ok_result,
        Err(PeerTubeError::ApiError(error, status)) => {
            if error.code == "invalid_client" {
                tracing::info!("got an invalid_client response, fetching the latest client config and trying again");
                clear_client_config(state).await;
                let client_config = read_or_fetch_client_config(state)
                    .await
                    .map_err(PeerTubeError::ClientError)?;

                let second_result =
                    send_token_request(state, client_config, username, auth_method, auth_secret)
                        .await;
                check_peertube_response(second_result).await?
            } else {
                tracing::error!(?error, "failed fetching oauth token");
                return Err(PeerTubeError::ApiError(error, status));
            }
        }
        Err(other) => return Err(other),
    };

    let status = ok_res.status();
    let token: OAuthToken = ok_res
        .json()
        .await
        .map_err(|err| PeerTubeError::JsonParsingFailed(status, err))?;
    Ok(token)
}

async fn read_or_fetch_client_config(
    state: &AppState,
) -> Result<OAuthClientConfig, reqwest::Error> {
    let mut cfg_guard = state.client_config.write().await;
    if cfg_guard.is_none() {
        let client_res = state
            .http_client
            .get(
                state
                    .peertube_url
                    .join("api/v1/oauth-clients/local")
                    .expect("invalid base url"),
            )
            .send()
            .await?;
        let client_res = client_res.error_for_status()?;
        let cfg: OAuthClientConfig = client_res.json().await?;

        *cfg_guard = Some(cfg);
    }
    Ok(cfg_guard.clone().unwrap())
}

async fn clear_client_config(state: &AppState) {
    let mut cfg_guard = state.client_config.write().await;
    *cfg_guard = None;
}
