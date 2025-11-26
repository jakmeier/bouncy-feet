use crate::{
    peertube::{
        token::{fetch_api_token_from_native_user, OAuthToken},
        PeerTubeError,
    },
    AppState,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct PeerTubeSystemUser {
    pub(crate) user_name: String,
    pub(crate) password: String,
    token: Arc<RwLock<Option<OAuthToken>>>,
}

impl PeerTubeSystemUser {
    pub fn new(user_name: String, password: String) -> Self {
        Self {
            user_name,
            password,
            token: Default::default(),
        }
    }

    pub async fn authorization_header(&self, state: &AppState) -> Result<String, PeerTubeError> {
        let guard = self.token.read().await;
        if let Some(token) = guard.as_ref() {
            return Ok(token.bearer_string());
        }
        drop(guard);

        let mut mut_guard = self.token.write().await;
        // second check: another task could have fetched the token in the
        // meantime, with this check we ensure only one task fetches it
        if mut_guard.is_none() {
            let new_token =
                fetch_api_token_from_native_user(state, &self.user_name, &self.password).await?;

            *mut_guard = Some(new_token);
        }

        Ok(mut_guard.as_ref().expect("just inserted").bearer_string())
    }

    pub(crate) async fn clear_token(&self, failed_token: &str) {
        let mut guard = self.token.write().await;
        if guard
            .as_ref()
            .is_some_and(|current| current.bearer_string() == failed_token)
        {
            *guard = None;
        }
    }
}
