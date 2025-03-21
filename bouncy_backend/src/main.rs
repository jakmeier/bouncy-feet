use axum::http::header;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::{get, post};
use axum::{middleware, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::AllowOrigin;

mod auth;
mod client_session;
mod user2;
mod user_meta;

/// Immutable shared state, should be cheap to clone.
#[derive(Clone)]
struct AppState {
    app_url: String,
    pg_db_pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let api_url = require_env("API_URL");
    let app_url = require_env("CLIENT_URL");
    // let oidc_issuer = require_env("OIDC_ISSUER");
    // let oidc_client_id = require_env("OIDC_CLIENT_ID");
    // let oidc_client_secret = require_env("OIDC_CLIENT_SECRET");
    let db_url = require_env("DATABASE_URL");

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
        app_url,
        pg_db_pool,
    };

    // let session_store = MemoryStore::default();
    // let session_layer = SessionManagerLayer::new(session_store)
    //     // absolutely don't leak the session id, authentication relies on it!
    //     .with_secure(true)
    //     // js access is not needed
    //     .with_http_only(true)
    //     // we need cross-origin requests from the PWA to the API sub domains
    //     .with_same_site(SameSite::None)
    //     // .with_domain(domain)
    //     .with_expiry(Expiry::OnInactivity(Duration::hours(24)));

    // axum: ServiceBuilder reverses order of middlewares (yay)
    // -> so these here run top to bottom, layers affect only routes afterwards
    // let oidc_login_service = ServiceBuilder::new()
    //     .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
    //         e.into_response()
    //     }))
    //     .layer(OidcLoginLayer::<AdditionalClaims>::new());

    let user_service = middleware::from_fn_with_state(state.clone(), user2::user_lookup);

    // let parsed_api_url = Uri::from_maybe_shared(api_url).expect("valid api url");
    let auth_service = ServiceBuilder::new()
        // .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
        //     e.into_response()
        // }))
        // .layer(
        //     OidcAuthLayer::<AdditionalClaims>::discover_client(
        //         parsed_api_url,
        //         oidc_issuer,
        //         oidc_client_id,
        //         Some(oidc_client_secret),
        //         vec![
        //             "openid".to_string(),
        //             "email".to_string(),
        //             "profile".to_string(),
        //         ],
        //     )
        //     .await
        //     .unwrap(),
        // )
        .layer(user_service);

    let client_origin = HeaderValue::from_str(&state.app_url).expect("url should be valid origin");
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

    let unauthenticated_app = Router::new()
        //auth is the endpoint for OICD code exchange
        .route("/", get(root))
        // .route("/auth", get(auth::oauth_callback))
        .route(
            "/new_guest_session",
            get(client_session::create_guest_session),
        );

    let authenticated_app = Router::new()
        .route("/user", get(user2::user_info))
        .route("/user/meta", get(user_meta::metadata))
        .route("/user/meta/update", post(user_meta::update_user_metadata))
        .route(
            "/new_guest_activity",
            post(client_session::record_guest_activity),
        );

    // Not used, yet. Only guests possible.
    // .layer(oidc_login_service) // enforces logging in

    let app = Router::new()
        .merge(unauthenticated_app)
        .merge(authenticated_app.layer(auth_service))
        // .layer(ServiceBuilder::new().layer(session_layer).layer(cors_layer))
        // .layer(session_layer)
        .layer(cors_layer)
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
