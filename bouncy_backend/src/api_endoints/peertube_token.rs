use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::{extract::State, response::IntoResponse, Json};
use axum_oidc::OidcAccessToken;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct TokenExchangeResponse {
    externalAuthToken: String,
    username: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct OAuthToken {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct OAuthClientConfig {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct PeertubeApiError {
    code: String,
}

#[axum::debug_handler]
pub async fn peertube_token_exchange(
    token: OidcAccessToken,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    // Security checks
    match headers.get(axum::http::header::CONTENT_TYPE) {
        Some(value) if value == "application/json" => (),
        Some(value) => {
            return (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                format!("unsupported content-type: {:?}", value),
            )
                .into_response()
        }
        None => {
            return (StatusCode::BAD_REQUEST, "missing content-type".to_string()).into_response()
        }
    }

    // Exchange JWT for bypass token
    let bypass_token_result = fetch_bypass_token(&state, &token).await;
    let bypass_token = match bypass_token_result {
        Ok(r) => r,
        Err(err) => {
            tracing::warn!(?err, "error fetching PeerTube bypass token");
            return (
                StatusCode::BAD_GATEWAY,
                "fetching PeerTube bypass token failed",
            )
                .into_response();
        }
    };

    // Request PeerTube OAuth token
    let peertube_token = match fetch_api_token(&state, bypass_token).await {
        Ok(r) => r,
        Err(err) => {
            tracing::warn!(?err, "error exchanging bypass for OAuth token");
            return (
                StatusCode::BAD_GATEWAY,
                "fetching PeerTube API token failed",
            )
                .into_response();
        }
    };

    Json(peertube_token).into_response()
}

async fn fetch_bypass_token(
    state: &AppState,
    token: &OidcAccessToken,
) -> Result<TokenExchangeResponse, anyhow::Error> {
    let exchange_url = state
        .peertube_url
        .join("plugins/auth-openid-connect/router/token-exchange")
        .expect("invalid base url");

    let res = state
        .http_client
        .post(exchange_url.as_str())
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &token.0),
        ])
        .send()
        .await?;

    let ok_res = res.error_for_status()?;
    if ok_res
        .headers()
        .get("Content-Type")
        .and_then(|hv| HeaderValue::to_str(hv).ok())
        .is_some_and(|hv| hv.contains("json"))
    {
        let json = ok_res.json().await?;
        Ok(json)
    } else {
        anyhow::bail!("PeerTube did not return a JSON response")
    }
}

async fn fetch_api_token(
    state: &AppState,
    bypass_token: TokenExchangeResponse,
) -> Result<OAuthToken, anyhow::Error> {
    // Fetch an OAuth token for the user that's usable by the API
    async fn send_token_request(
        state: &AppState,
        client_config: OAuthClientConfig,
        bypass_token: &TokenExchangeResponse,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let token_url = state
            .peertube_url
            .join("api/v1/users/token")
            .expect("invalid base url");
        state
            .http_client
            .post(token_url.as_str())
            .form(&[
                ("username", bypass_token.username.as_str()),
                ("externalAuthToken", bypass_token.externalAuthToken.as_str()),
                ("client_id", client_config.client_id.as_str()),
                ("client_secret", client_config.client_secret.as_str()),
                ("grant_type", "password"),
            ])
            .send()
            .await
    }

    let client_config = read_or_fetch_client_config(state).await?;
    let mut token_res = send_token_request(state, client_config, &bypass_token).await?;

    // error details are reported with content type "application/problem+json; charset=utf-8"
    // checking just for "json" for robustness to changes in how the content header is set by the server
    if token_res.status() == reqwest::StatusCode::BAD_REQUEST
        && token_res
            .headers()
            .get("Content-Type")
            .and_then(|hv| HeaderValue::to_str(hv).ok())
            .is_some_and(|hv| hv.contains("json"))
    {
        let error: PeertubeApiError = token_res.json().await?;
        if error.code == "invalid_client" {
            tracing::info!("got an invalid_client response, fetching the latest client config and trying again");
            clear_client_config(state).await;
            let client_config = read_or_fetch_client_config(state).await?;
            token_res = send_token_request(state, client_config, &bypass_token).await?;
        } else {
            anyhow::bail!("failed fetching oauth token {error:?}");
        }
    }
    let token_res = token_res.error_for_status()?;

    let token: OAuthToken = token_res.json().await?;
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
