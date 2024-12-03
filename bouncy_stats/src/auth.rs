//! Authentication flow:
//!
//! Bouncy Feet uses OIDC with the Authentication Code flow. The PWA (client)
//! redirects to Keycloak for the login, which then redirects to the
//! oauth_callback in this file.
//!
//! The oauth_callback will establish a session with the user, without revealing
//! the the access token to the user itself. Then it redirects back to the PWA's
//! domain, where authenticated API requests are now possible.

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

/// The IdP redirect here for establishing a session.
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
    // (dev or app origin, with exact fully qualified url)
    Redirect::to(&state.app_url).into_response()
}
