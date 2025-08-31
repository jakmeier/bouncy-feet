use crate::auth::login;
use crate::layers::oidc::{oidc_auth_layer, oidc_login_layer};
use crate::layers::session::in_memory_cookie_session_layer;
use axum::error_handling::HandleErrorLayer;
use axum::http::header;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Router};
use axum_oidc::error::MiddlewareError;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::AllowOrigin;
use tracing::Level;
use url::Url;

pub(crate) mod auth;
pub(crate) mod client_session;
pub(crate) mod dance_activity;
pub(crate) mod layers;
pub(crate) mod stats;
pub(crate) mod user;
pub(crate) mod user_meta;

/// Immutable shared state, should be cheap to clone.
#[derive(Clone)]
struct AppState {
    app_url: Url,
    pg_db_pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let api_url = require_env("API_URL");
    let app_url = require_env("CLIENT_URL");
    let oidc_issuer = require_env("OIDC_ISSUER");
    let oidc_client_id = require_env("OIDC_CLIENT_ID");
    let oidc_client_secret = require_env("OIDC_CLIENT_SECRET");
    let db_url = require_env("DATABASE_URL");

    tracing::info!("My domain:  {}", api_url);
    tracing::info!("App domain: {}", app_url);

    let parsed_api_url = Url::parse(&api_url).unwrap();
    let parsed_app_url = Url::parse(&app_url).unwrap();

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

    let state = AppState {
        app_url: parsed_app_url,
        pg_db_pool,
    };

    let user_service = middleware::from_fn_with_state(state.clone(), user::user_lookup);
    let session_layer = in_memory_cookie_session_layer(&state.app_url);
    let login_layer = oidc_login_layer();
    let auth_layer = oidc_auth_layer(
        &parsed_api_url,
        oidc_issuer,
        oidc_client_id,
        oidc_client_secret,
    )
    .await;

    let client_origin =
        HeaderValue::from_str(state.app_url.as_str()).expect("url should be valid origin");
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
            e.into_response()
        }))
        .layer(auth_layer)
        .layer(user_service);

    // top-to-bottom order for service builder
    let login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(login_layer);

    // bottom-to-top order for router
    let unauthenticated_app = Router::new()
        .route("/", get(root))
        .route(
            "/new_guest_session",
            get(client_session::create_guest_session),
        )
        .route("/stats", get(stats::stats));

    // bottom-to-top order for router
    let authenticated_app = Router::new()
        // The login server only apples on this specific route.
        // This forwards to Keycloak, while all other routes
        // will return an UNAUTHORIZED response.
        .route("/login", get(login))
        .layer(login_service)
        .route("/user", get(user::user_info))
        .route("/user/meta", get(user_meta::metadata))
        .route("/user/meta/update", post(user_meta::update_user_metadata))
        .route(
            "/new_guest_activity",
            post(client_session::record_guest_activity),
        )
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
    eprintln!("Internal error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

fn require_env(var_name: &str) -> String {
    std::env::var(var_name)
        .unwrap_or_else(|err| panic!("missing {var_name} environment variable, err {err}"))
}
