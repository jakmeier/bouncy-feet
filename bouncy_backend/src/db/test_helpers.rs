//! Shared test helpers for database tests.
//!
//! This module provides common utilities for setting up and tearing down test database state.

use crate::api_endoints::auth::KeycloakClientConfig;
use crate::cache::DataCache;
use crate::peertube::system_user::PeerTubeSystemUser;
use sqlx::PgPool;
use std::sync::Arc;
use url::Url;

/// Build a minimal AppState for testing.
///
/// Only the `pg_db_pool` field is meaningful; all other fields use dummy values.
pub fn make_test_state(pool: PgPool) -> crate::AppState {
    crate::AppState {
        app_url: "http://localhost:3000".parse::<Url>().unwrap(),
        api_url: "http://localhost:4000".parse::<Url>().unwrap(),
        peertube_url: "http://localhost:9000".parse::<Url>().unwrap(),
        pg_db_pool: pool,
        http_client: reqwest::Client::new(),
        peertube_client_config: Arc::new(tokio::sync::RwLock::new(None)),
        kc_config: KeycloakClientConfig {
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            registration_url: "http://localhost:8080/register".parse::<Url>().unwrap(),
            logout_url: "http://localhost:8080/logout".parse::<Url>().unwrap(),
        },
        system_user: PeerTubeSystemUser::new("system_user".to_string(), "password".to_string()),
        data_cache: DataCache::default(),
    }
}
