//! Replaces sqlite based users.

use crate::layers::oidc::AdditionalClaims;
use crate::user::UserId;
use axum::Extension;
use axum_oidc::OidcClaims;
use serde_json::json;

pub async fn user_info(
    claims: Option<OidcClaims<AdditionalClaims>>,
    Extension(user_id): Extension<UserId>,
) -> String {
    if let Some(claims) = claims {
        json!({
            "id": user_id.num(),
            "sub": claims.subject().as_str()
        })
    } else {
        json!({
            "id": user_id.num(),
            "sub": null
        })
    }
    .to_string()
}
