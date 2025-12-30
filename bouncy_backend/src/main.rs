use crate::api_endoints::auth::KeycloakClientConfig;
use crate::api_endoints::peertube_token::{self, peertube_token_exchange};
use crate::cache::ClubsCache;
use crate::layers::oidc::{oidc_auth_layer, oidc_login_layer};
use crate::layers::session::pg_backed_cookie_session_layer;
use crate::peertube::system_user::PeerTubeSystemUser;
use axum::error_handling::HandleErrorLayer;
use axum::http::header;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Router};
use axum_oidc::error::MiddlewareError;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::AllowOrigin;
use tracing::Level;
use url::Url;

pub(crate) mod api_endoints;
pub(crate) mod cache;
pub(crate) mod db;
pub(crate) mod layers;
pub(crate) mod peertube;

// re-export
pub(crate) use db::*;

/// Immutable shared state, should be cheap to clone.
#[derive(Clone)]
struct AppState {
    app_url: Url,
    api_url: Url,
    peertube_url: Url,
    pg_db_pool: PgPool,
    http_client: reqwest::Client,
    peertube_client_config: Arc<tokio::sync::RwLock<Option<peertube_token::OAuthClientConfig>>>,
    kc_config: KeycloakClientConfig,
    system_user: PeerTubeSystemUser,
    clubs_cache: ClubsCache,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let api_url = require_env("API_URL");
    let app_url = require_env("CLIENT_URL");
    let peertube_url = Url::parse(&require_env("PEERTUBE_URL"))?;
    let oidc_issuer = require_env("OIDC_ISSUER");
    let oidc_client_id = require_env("OIDC_CLIENT_ID");
    let oidc_client_secret = require_env("OIDC_CLIENT_SECRET");
    let db_url = require_env("DATABASE_URL");
    let system_user = require_env("PEERTUBE_SYSTEM_USER");
    let system_user_pw = require_env("PEERTUBE_SYSTEM_USER_PASSWORD");

    tracing::info!("My domain:  {}", api_url);
    tracing::info!("App domain: {}", app_url);

    let parsed_api_url = Url::parse(&api_url).unwrap();
    let parsed_app_url = Url::parse(&app_url).unwrap();

    // `prompt=create` is the recommended way to open the registration page.
    // Requires Keycloak 26.1.0 or newer.
    let kc_registration_url = Url::parse(&format!(
        "{oidc_issuer}/protocol/openid-connect/auth?prompt=create&client_id={oidc_client_id}&response_type=code&scope=openid"
    ))
    .unwrap();

    let kc_logout_url = Url::parse(&format!(
        "{oidc_issuer}/protocol/openid-connect/logout?client_id={oidc_client_id}"
    ))
    .unwrap();

    let pg_db_pool = PgPool::connect(&db_url).await?;

    // TODO: better DB setup
    {
        println!("Starting DB migrations...");
        sqlx::migrate!("./db_migrations/")
            .run(&pg_db_pool)
            .await
            .unwrap();
        println!("...DB migrations done");
    }

    let http_client = reqwest::Client::new();

    let system_user = PeerTubeSystemUser::new(system_user, system_user_pw);

    let state = AppState {
        app_url: parsed_app_url.clone(),
        api_url: parsed_api_url.clone(),
        peertube_url,
        pg_db_pool,
        http_client,
        peertube_client_config: Arc::default(),
        kc_config: KeycloakClientConfig {
            client_id: oidc_client_id.clone(),
            client_secret: oidc_client_secret.clone(),
            registration_url: kc_registration_url,
            logout_url: kc_logout_url,
        },
        system_user,
        clubs_cache: Default::default(),
    };

    let user_service = middleware::from_fn_with_state(state.clone(), layers::user::user_lookup);
    let session_layer =
        pg_backed_cookie_session_layer(&parsed_api_url.clone(), state.pg_db_pool.clone());
    let login_layer = oidc_login_layer();
    let auth_layer = oidc_auth_layer(
        &parsed_api_url,
        oidc_issuer,
        oidc_client_id,
        oidc_client_secret,
    )
    .await;

    // must use origin without trailing slash
    let client_origin = HeaderValue::from_str(&parsed_app_url.origin().ascii_serialization())
        .expect("url should be valid origin");
    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_origin(AllowOrigin::exact(client_origin))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT_LANGUAGE,
            header::CONTENT_LANGUAGE,
            header::COOKIE,
        ])
        .allow_credentials(true);

    // top-to-bottom order for service builder
    let auth_service = ServiceBuilder::new()
        .layer(session_layer)
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            tracing::warn!("Auth layer: {:?}", e);
            e.into_response()
        }))
        .layer(auth_layer)
        .layer(user_service);

    // top-to-bottom order for service builder
    let login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            tracing::warn!("Login layer: {:?}", e);
            e.into_response()
        }))
        .layer(login_layer);

    // bottom-to-top order for router
    let unauthenticated_app = Router::new()
        .route("/", get(root))
        .route("/clubs", get(api_endoints::club::clubs))
        .route("/clubs/{club_id}", get(api_endoints::club::club))
        .route("/register", get(api_endoints::auth::register))
        .route("/logout", get(api_endoints::auth::logout))
        .route(
            "/new_guest_session",
            get(api_endoints::client_session::create_guest_session),
        )
        .route("/stats", get(api_endoints::stats::stats));

    // bottom-to-top order for router
    let login_route = Router::new()
        // The login service only applies on this specific route.
        // It forwards to Keycloak.
        .route("/login", get(api_endoints::auth::login))
        .layer(login_service);

    // bottom-to-top order for router
    // All protected routes will return an UNAUTHORIZED response if
    // authentication fails.
    let protected_app = Router::new()
        .route("/peertube/token", post(peertube_token_exchange))
        .route("/clubs/create", post(api_endoints::club::create_club))
        .route("/clubs/joined", get(api_endoints::club::my_clubs))
        .route(
            "/clubs/add_member",
            post(api_endoints::club::add_club_member),
        )
        .route("/clubs/video/add", post(api_endoints::club::add_video))
        .route(
            "/clubs/{club_id}/private",
            get(api_endoints::club::club_with_private_details),
        )
        .route("/clubs/{club_id}", post(api_endoints::club::update_club))
        .route(
            "/clubs/{club_id}/avatar",
            post(api_endoints::club::update_avatar),
        )
        .route("/user", get(api_endoints::user::user_info))
        .route("/user/meta", get(api_endoints::user_meta::metadata))
        .route("/users", post(api_endoints::user::list_users))
        .route(
            "/user/meta/update",
            post(api_endoints::user_meta::update_user_metadata),
        )
        .route(
            "/new_guest_activity",
            post(api_endoints::client_session::record_guest_activity),
        )
        .layer(axum::middleware::from_fn(
            layers::user::require_user_service,
        ));

    // auth_service must run before protected routes as well as login
    let authenticated_app = Router::new()
        .merge(login_route)
        .merge(protected_app)
        .layer(auth_service);

    // bottom-to-top order for router
    let app = Router::new()
        .merge(unauthenticated_app)
        .merge(authenticated_app)
        .layer(cors_layer)
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(Level::INFO)),
        )
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> String {
    format!(
        "Bouncy Feet stats API server running.\nPackage version: {}",
        env!("CARGO_PKG_VERSION")
    )
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("Internal error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

fn require_env(var_name: &str) -> String {
    std::env::var(var_name)
        .unwrap_or_else(|err| panic!("missing {var_name} environment variable, err {err}"))
}
