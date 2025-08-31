use axum_oidc::EmptyAdditionalClaims;
use axum_oidc::{OidcAuthLayer, OidcLoginLayer};
use url::Url;

pub(crate) type AdditionalClaims = EmptyAdditionalClaims;

/// Enforce and redirect to a login page on handles with this service.
///
/// Requires [`oidc_auth_layer`] before this layer.
pub fn oidc_login_layer() -> OidcLoginLayer<AdditionalClaims> {
    OidcLoginLayer::<AdditionalClaims>::new()
}

/// Provide claims on requests for extractors
pub async fn oidc_auth_layer(
    api_url: &Url,
    oidc_issuer: String,
    oidc_client_id: String,
    oidc_client_secret: String,
) -> OidcAuthLayer<AdditionalClaims> {
    OidcAuthLayer::<AdditionalClaims>::discover_client(
        api_url.as_str().parse().expect("Invalid URI"),
        oidc_issuer,
        oidc_client_id,
        Some(oidc_client_secret),
        vec![
            "openid".to_string(),
            "email".to_string(),
            "profile".to_string(),
        ],
    )
    .await
    .unwrap()
}
