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
use axum_oidc::{ClearSessionFlag, EmptyAdditionalClaims, OidcClaims};
use url::Url;

use crate::AppState;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    redirect_back_to: Option<Url>,
}

#[derive(Clone)]
pub struct KeycloakClientConfig {
    #[allow(unused)]
    pub client_id: String,
    #[allow(unused)]
    pub client_secret: String,
    pub registration_url: Url,
    pub logout_url: Url,
}

/// Calling this will redirect to Keycloak, have the user log in and then
/// redirect back to the PWA domain.
#[axum::debug_handler]
pub async fn login(
    query: Query<QueryParams>, //< order important, query must be before claims for registration to succeed in parsing both
    claims: OidcClaims<EmptyAdditionalClaims>,
    State(state): State<AppState>,
) -> Response {
    // The main checks are done in the OIDC middleware.
    // Here we just add additional checks.
    if !claims.email_verified().unwrap_or_default() {
        return (StatusCode::FORBIDDEN, "email not verified").into_response();
    }

    // Redirect to the PWA frontend if login was successful and redirect parameter was provided
    if let Some(redirect_back_to) = &query.0.redirect_back_to {
        return safe_redirect_to_app(&state, redirect_back_to).into_response();
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

/// Calling this will redirect to the Keycloak registration page.
///
/// If `redirect_back_to` is set, the user is redirected to where they were
/// after a successful registration, going through a flow as follows.
///
/// api/register
/// -> kc/auth
/// -> api/login
/// -> redirect_back_to
///
/// Registration is outside the standard oidc flow, so it's not implemented in
/// axum_oidc. But Keycloak does follow a "prompt" extension which has a formal
/// spec here: https://openid.net/specs/openid-connect-prompt-create-1_0.html
///
/// Keycloak documentation:
/// https://www.keycloak.org/docs/latest/server_admin/#_registration-rc-client-flows
///
/// TLDR: set `prompt=create` to go directly to register, rather than the login page.
#[axum::debug_handler]
pub async fn register(State(state): State<AppState>, query: Query<QueryParams>) -> Response {
    // Form a URL where Keycloak can redirect back, so that the backend can
    // redirect the user to where they were in the app.
    let mut redirect_after_register = state
        .api_url
        .join("/login")
        .expect("parsing configured register redirect url failed");
    if let Some(redirect_back_to) = &query.redirect_back_to {
        redirect_after_register
            .query_pairs_mut()
            .append_pair("redirect_back_to", redirect_back_to.as_str());
    }

    // Take the preformed registration URL and append the `redirect_uri` param.
    let mut url = state.kc_config.registration_url.clone();
    url.query_pairs_mut()
        .append_pair("redirect_uri", redirect_after_register.as_str());

    Redirect::to(url.as_str()).into_response()
}

#[axum::debug_handler]
pub async fn logout(
    query: Query<QueryParams>,
    // needed to ensure the oidc auth layer is active, otherwise clearing a
    // session won't do anything
    _claims: OidcClaims<EmptyAdditionalClaims>,
    State(state): State<AppState>,
) -> Response {
    let mut url = state.kc_config.logout_url.clone();
    if let Some(redirect_back_to) = &query.redirect_back_to {
        url.query_pairs_mut()
            .append_pair("post_logout_redirect_uri", redirect_back_to.as_str());
    }

    (
        axum::Extension(ClearSessionFlag),
        Redirect::to(url.as_str()),
    )
        .into_response()
}

fn safe_redirect_to_app(state: &AppState, redirect_back_to: &Url) -> Redirect {
    if redirect_back_to.origin() == state.app_url.origin() {
        Redirect::to(redirect_back_to.as_str())
    } else {
        tracing::warn!("Refusing to redirect to {}", redirect_back_to);
        Redirect::to(state.app_url.as_str())
    }
}
