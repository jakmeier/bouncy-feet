use axum_oidc::{OidcAuthLayer, OidcLoginLayer};
use std::collections::BTreeMap;
use url::Url;

/// Keycloak's native client-role representation in `resource_access`.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
struct ClientAccess {
    #[serde(default)]
    roles: Vec<String>,
}

/// Additional ID-token claims consumed by the backend. Client roles remain
/// scoped to their Keycloak client; no custom protocol mapper is needed.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub(crate) struct AdditionalClaims {
    #[serde(default)]
    resource_access: BTreeMap<String, ClientAccess>,
}

impl AdditionalClaims {
    pub(crate) fn has_client_role(&self, client_id: &str, role: &str) -> bool {
        self.resource_access
            .get(client_id)
            .is_some_and(|access| access.roles.iter().any(|candidate| candidate == role))
    }
}

impl axum_oidc::AdditionalClaims for AdditionalClaims {}
impl openidconnect::AdditionalClaims for AdditionalClaims {}

/// Enforce and redirect to a login page on handles with this service.
///
/// Requires [`oidc_auth_layer`] before this layer.
pub fn oidc_login_layer() -> OidcLoginLayer<AdditionalClaims> {
    OidcLoginLayer::<AdditionalClaims>::new()
}

/// Provide claims on requests for extractors.
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

#[cfg(test)]
mod tests {
    use super::AdditionalClaims;

    #[test]
    fn recognizes_only_the_role_for_the_configured_client() {
        let claims: AdditionalClaims = serde_json::from_value(serde_json::json!({
            "resource_access": {
                "bouncy-dev-backend": { "roles": ["qr-admin"] },
                "account": { "roles": ["manage-account"] }
            }
        }))
        .unwrap();

        assert!(claims.has_client_role("bouncy-dev-backend", "qr-admin"));
        assert!(!claims.has_client_role("account", "qr-admin"));
        assert!(!claims.has_client_role("bouncy-backend", "qr-admin"));
    }
}
