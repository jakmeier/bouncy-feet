//! Shared test helpers for database tests.
//!
//! This module provides common utilities for setting up and tearing down test database state.

use crate::api_endoints::auth::KeycloakClientConfig;
use crate::api_endoints::user_meta::{update_user_metadata, UpdateMetaDataRequest};
use crate::cache::DataCache;
use crate::peertube::system_user::PeerTubeSystemUser;
use crate::user::{User, UserId};
use axum::extract::State;
use axum::{Extension, Json};
use chrono::DateTime;
use sqlx::PgPool;
use std::sync::Arc;
use url::Url;
use uuid::Uuid;

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

pub async fn create_new_full_user(state: &crate::AppState) -> User {
    let sub = Uuid::new_v4().to_string();
    User::lookup_by_oidc_or_create(state, &sub).await
}

pub async fn create_new_full_user_det(
    state: &crate::AppState,
    seed: u64,
    display_name: Option<String>,
) -> User {
    let sub = Uuid::from_u64_pair(0, seed).to_string();
    let user = User::lookup_by_oidc_or_create(state, &sub).await;
    if let Some(name) = display_name {
        set_display_name(state, user.clone(), name).await;
    }
    user
}

pub async fn create_new_guest_user_det(
    state: &crate::AppState,
    display_name: Option<String>,
) -> UserId {
    let id = UserId::create_new_guest(&state.pg_db_pool).await;
    if let Some(name) = display_name {
        let user = User {
            id,
            // These don't matter for `update_user_metadata`
            oidc_subject: None,
            peertube_account_id: None,
            peertube_handle: None,
        };

        set_display_name(state, user, name).await;
    }
    id
}

async fn set_display_name(state: &crate::AppState, user: User, name: String) {
    let request = UpdateMetaDataRequest {
        key_name: "s:publicName".to_owned(),
        key_value: name,
        // Use fixed date to use in deterministic testing
        last_modified: DateTime::from_timestamp(1767227696, 789).unwrap(),
        version: 0,
    };

    update_user_metadata(Extension(user), State(state.clone()), Json(request))
        .await
        .expect("setting public name must succeed");
}

pub async fn setup_deterministic_users(
    state: &crate::AppState,
    num_guests: u32,
    num_full_users: u32,
    seed: u64,
) -> u64 {
    for i in 0..num_guests {
        let user_seed = seed + i as u64;
        let display_name = format!("Noob Dancer {}", user_seed);
        create_new_guest_user_det(state, Some(display_name)).await;
    }

    for i in 0..num_full_users {
        let user_seed = seed + i as u64;
        let display_name = format!("Pro Dancer {}", user_seed);
        create_new_full_user_det(state, user_seed, Some(display_name)).await;
    }

    seed + num_full_users as u64
}
