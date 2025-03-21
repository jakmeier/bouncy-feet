//! Authentication flow:
//!
//! Bouncy Feet uses OIDC with the Authentication Code flow. The PWA (client)
//! establishes a session with the API backend and may call the `oauth_callback`
//! endpoint to authenticate the session.
//!
//! The oauth_callback will establish a session with the user, without revealing
//! the access token to the user itself. Then it redirects back to the PWA's
//! domain, where authenticated API requests are now possible.
//!
//! Guest users can be authenticated by a client secret included in the body as JSON.

use crate::AppState;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::State, http::StatusCode};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims};
use serde::{Deserialize, Serialize};

pub(crate) type AdditionalClaims = EmptyAdditionalClaims;

#[derive(Deserialize, Serialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: u64,
    token_type: String,
}

/// Calling this will redirect to Keyloak, have the user log in and then
/// redirect back to the PWA domain.
pub async fn oauth_callback(
    claims: OidcClaims<AdditionalClaims>,
    State(state): State<AppState>,
) -> Response {
    // The main checks are done in the OIDC middleware.
    // Here we just add additional checks.
    if !claims.email_verified().unwrap_or_default() {
        return (StatusCode::FORBIDDEN, "email not verified").into_response();
    }

    // Redirect to the PWA frontend if login was successful
    // TODO: redirect to the exact same page the user was on before
    Redirect::to(&state.app_url).into_response()
}
