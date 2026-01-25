//! System-Moderator connection to PeerTube instance, used to manage resources
//! non-personal resources, such as club playlists.

use crate::AppState;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures::future::BoxFuture;
use reqwest::header::HeaderValue;

pub(crate) mod channel;
pub(crate) mod playlist;
pub(crate) mod system_user;
pub(crate) mod token;
pub(crate) mod user;

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
    #[error("no json response provided with a successful response, status was {0}")]
    NoJsonResponse(reqwest::StatusCode, String),
    #[error("json response provided with a successful response could not be decoded, status was {0}, error was {1}")]
    JsonParsingFailed(reqwest::StatusCode, reqwest::Error),
    #[error("error sending request {0}")]
    ClientError(#[source] reqwest::Error),
    #[error("invalid value {0}")]
    ClientValidationError(String),
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
        if let Ok(error_text) = response.text().await {
            tracing::warn!(?error_text, ?status, "Non-Json error from PeerTube");
        }
        Err(PeerTubeError::NoJsonError(status))
    }
}

/// Returns true if a retry would make sense
pub(crate) async fn handle_peertube_error(state: &AppState, err: &PeerTubeError) -> RetryHint {
    match err {
        PeerTubeError::SystemAuthFailed(failed_token) => {
            // Note: We could also refresh in most cases. But since the pw of
            // the system user is stored anyway, there is no benefit of a
            // refresh. It would just be more code that needs to be maintained.
            state.system_user.clear_token(failed_token).await;
            RetryHint::RetryNow
        }
        PeerTubeError::NoJsonError(reqwest::StatusCode::BAD_GATEWAY) => {
            // There can be temporary problems on the network.
            RetryHint::RetryAfter(std::time::Duration::from_millis(1000))
        }
        _ => RetryHint::NoRetry,
    }
}

/// Retry a PeerTube operation a few times and then return an error response if
/// it didn't succeed.
///
/// `Op``: a closure producing a boxed future that may borrow `AppState` and
/// other locals
pub async fn retry_peertube_op<'s, T, Op>(
    state: &'s AppState,
    mut op: Op,
) -> Result<T, axum::response::Response>
where
    Op: FnMut(&'s AppState) -> BoxFuture<'s, Result<T, PeerTubeError>>,
    T: 's,
{
    let mut remaining_attempts = 5;
    loop {
        match op(state).await {
            Ok(v) => return Ok(v),
            Err(e) => {
                let should_retry = handle_peertube_error(state, &e).await;
                tracing::error!(?e, ?remaining_attempts, "PeerTube operation failed");

                if remaining_attempts <= 0 {
                    let resp =
                        (StatusCode::BAD_GATEWAY, "PeerTube service failure").into_response();
                    return Err(resp);
                }
                remaining_attempts -= 1;

                match should_retry {
                    RetryHint::NoRetry => {
                        let resp =
                            (StatusCode::BAD_GATEWAY, "PeerTube service failure").into_response();
                        return Err(resp);
                    }
                    RetryHint::RetryNow => (),
                    RetryHint::RetryAfter(delay) => {
                        tokio::time::sleep(delay).await;
                    }
                }
                remaining_attempts -= 1;
            }
        }
    }
}

pub(crate) enum RetryHint {
    NoRetry,
    RetryNow,
    RetryAfter(std::time::Duration),
}
