use crate::peertube::token::fetch_api_token_from_bypass_token;
use crate::peertube::{
    self, check_peertube_system_user_response, retry_peertube_op, PeerTubeError,
};
use crate::user::User;
use crate::AppState;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::{extract::State, response::IntoResponse, Json};
use axum_oidc::OidcAccessToken;
use futures::FutureExt;
use reqwest::header::HeaderValue;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct TokenExchangeResponse {
    pub externalAuthToken: String,
    pub username: String,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct OAuthClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[axum::debug_handler]
pub async fn peertube_token_exchange(
    axum::Extension(me): axum::Extension<User>,
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

    // TODO: use caching and refreshing

    // Exchange JWT for bypass token
    let bypass_token_result =
        retry_peertube_op(&state, |s| fetch_bypass_token(s, &token).boxed()).await;

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
    let peertube_token_result = retry_peertube_op(&state, |s| {
        fetch_api_token_from_bypass_token(s, &bypass_token).boxed()
    })
    .await;

    let peertube_token = match peertube_token_result {
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

    // Lazily set the PeerTube user id.
    // It won't be updated for this request (not needed for token exchange) but
    // future requests will read the new value from the DB.
    let mut me = me.clone();
    let res = peertube::user::ensure_peertube_id(&state, &mut me, &peertube_token).await;
    if let Err(err) = res {
        tracing::warn!(?err, "Failed setting PeerTube user id.");
    }

    Json(peertube_token).into_response()
}

async fn fetch_bypass_token(
    state: &AppState,
    token: &OidcAccessToken,
) -> Result<TokenExchangeResponse, PeerTubeError> {
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
        .await;

    let ok_res = check_peertube_system_user_response(res, String::new()).await?;

    let status = ok_res.status();
    if ok_res
        .headers()
        .get("Content-Type")
        .and_then(|hv| HeaderValue::to_str(hv).ok())
        .is_some_and(|hv| hv.contains("json"))
    {
        let json = ok_res
            .json()
            .await
            .map_err(|err: reqwest::Error| PeerTubeError::JsonParsingFailed(status, err))?;
        Ok(json)
    } else {
        let err_msg = ok_res.text().await.unwrap_or_default();
        Err(PeerTubeError::NoJsonResponse(status, err_msg))
    }
}
