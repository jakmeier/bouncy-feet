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

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims};
use url::Url;

use crate::AppState;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    redirect_back_to: Option<Url>,
}

/// Calling this will redirect to Keyloak, have the user log in and then
/// redirect back to the PWA domain.
#[axum::debug_handler]
pub async fn login(
    claims: OidcClaims<EmptyAdditionalClaims>,
    State(state): State<AppState>,
    query: Query<QueryParams>,
) -> Response {
    // The main checks are done in the OIDC middleware.
    // Here we just add additional checks.
    if !claims.email_verified().unwrap_or_default() {
        return (StatusCode::FORBIDDEN, "email not verified").into_response();
    }

    // Redirect to the PWA frontend if login was successful and redirect parameter was provided
    if let Some(redirect_back_to) = query.0.redirect_back_to {
        if redirect_back_to.origin() == state.app_url.origin() {
            return Redirect::to(redirect_back_to.as_str()).into_response();
        } else {
            tracing::warn!("Refusing to redirect to {}", redirect_back_to);
            return Redirect::to(state.app_url.as_str()).into_response();
        }
    }

    let name = claims
        .preferred_username()
        .map(|name| name.to_string())
        .unwrap_or_else(|| {
            claims
                .name()
                .and_then(|localized_name| localized_name.get(claims.locale()))
                .map(|name| name.to_string())
                .unwrap_or_default()
        });

    format!(
        "Hello {}\nBouncy Feet stats API server running.\nPackage version: {}",
        name,
        env!("CARGO_PKG_VERSION")
    )
    .into_response()
}
