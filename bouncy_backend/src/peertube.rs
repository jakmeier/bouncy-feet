//! System-Moderator connection to PeerTube instance, used to manage resources
//! non-personal resources, such as club playlists.

use crate::AppState;
use reqwest::header::HeaderValue;

pub(crate) mod playlist;
pub(crate) mod system_user;
pub(crate) mod token;

#[derive(serde::Deserialize, Clone, Debug)]
pub(crate) struct PeertubeApiErrorResponse {
    pub code: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum PeerTubeError {
    #[error("PeerTube returned an error {0:?} with status {1}")]
    ApiError(PeertubeApiErrorResponse, reqwest::StatusCode),
    #[error("json response error doesn't parse, status was {0}, full error {1}")]
    UnknownJsonError(reqwest::StatusCode, String),
    #[error("no json response provided with the error, status was {0}")]
    NoJsonError(reqwest::StatusCode),
    #[error("json response provided with a successful response could not be decoded, status was {0}, error was {1}")]
    JsonParsingFailed(reqwest::StatusCode, reqwest::Error),
    #[error("error sending request {0}")]
    ClientError(#[source] reqwest::Error),
    #[error("system auth failed")]
    SystemAuthFailed(String),
}

pub(crate) async fn check_peertube_system_user_response(
    response_result: Result<reqwest::Response, reqwest::Error>,
    used_access_token: String,
) -> Result<reqwest::Response, PeerTubeError> {
    match check_peertube_response(response_result).await {
        Err(PeerTubeError::ApiError(_, reqwest::StatusCode::UNAUTHORIZED)) => {
            Err(PeerTubeError::SystemAuthFailed(used_access_token))
        }
        other => other,
    }
}

pub(crate) async fn check_peertube_response(
    response_result: Result<reqwest::Response, reqwest::Error>,
) -> Result<reqwest::Response, PeerTubeError> {
    let Ok(response) = response_result else {
        return Err(PeerTubeError::ClientError(response_result.unwrap_err()));
    };
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    // Try decode JSON encoded error message
    // error details are reported with content type "application/problem+json; charset=utf-8"
    // checking just for "json" for robustness to changes in how the content header is set by the server
    let is_json = response
        .headers()
        .get("Content-Type")
        .and_then(|hv| HeaderValue::to_str(hv).ok())
        .is_some_and(|hv| hv.contains("json"));
    if is_json {
        let text = response.text().await.unwrap_or_default();
        let result: Result<PeertubeApiErrorResponse, _> = serde_json::from_str(&text);
        let typed_error = result.map_err(|_err| PeerTubeError::UnknownJsonError(status, text))?;
        Err(PeerTubeError::ApiError(typed_error, status))
    } else {
        Err(PeerTubeError::NoJsonError(status))
    }
}

/// Returns true if a retry would make sense
pub(crate) async fn handle_peertube_error(state: &AppState, err: &PeerTubeError) -> bool {
    if let PeerTubeError::SystemAuthFailed(failed_token) = err {
        // Note: We could also refresh in most cases. But since the pw of
        // the system user is stored anyway, there is no benefit of a
        // refresh. It would just be more code that needs to be maintained.
        state.system_user.clear_token(failed_token).await;
        return true;
    }
    false
}
