use auth::AdditionalClaims;
use axum::error_handling::HandleErrorLayer;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{extract, middleware, Router};
use axum_oidc::error::MiddlewareError;
use axum_oidc::{OidcAuthLayer, OidcClaims, OidcLoginLayer};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Executor, PgPool, Sqlite, SqlitePool};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::AllowOrigin;
use tower_sessions::cookie::{time::Duration, SameSite};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use user::{get_scores, post_stats};

mod auth;
mod user;
mod user2;
mod user_meta;

const DB_PATH: &str = "sqlite:data/db.sqlite";

/// Immutable shared state, should be cheap to clone.
#[derive(Clone)]
struct AppState {
    app_url: String,
    sqlite_db_pool: SqlitePool,
    pg_db_pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_url = require_env("API_URL");
    let app_url = require_env("CLIENT_URL");
    let oidc_issuer = require_env("OIDC_ISSUER");
    let oidc_client_id = require_env("OIDC_CLIENT_ID");
    let oidc_client_secret = require_env("OIDC_CLIENT_SECRET");
    let db_url = require_env("DATABASE_URL");

    let sqlite_db_pool = create_db_pool().await?;
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
        sqlite_db_pool,
        pg_db_pool,
    };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        // absolutely don't leak the session id, authentication relies on it!
        .with_secure(true)
        // js access is not needed
        .with_http_only(true)
        // we need cross-origin requests from the PWA to the API sub domains
        .with_same_site(SameSite::None)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)));

    let oidc_login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(OidcLoginLayer::<AdditionalClaims>::new());

    let parsed_api_url = Uri::from_maybe_shared(api_url).expect("valid api url");
    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(
            OidcAuthLayer::<AdditionalClaims>::discover_client(
                parsed_api_url,
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
            .unwrap(),
        );

    let client_origin = HeaderValue::from_str(&state.app_url).expect("url should be valid origin");
    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_origin(AllowOrigin::exact(client_origin))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    let user_layer = middleware::from_fn_with_state(state.clone(), user2::user_lookup);

    let app = Router::new()
        .route("/user/meta", get(user_meta::metadata))
        .route("/user/meta/update", post(user_meta::update_user_metadata))
        .route("/user", get(user2::user_info))
        .layer(user_layer)
        // /auth is the endpoint for OICD code exchange
        .route("/auth", get(auth::oauth_callback))
        .layer(oidc_login_service) // enforces logging in
        .route("/testauth", get(authenticated))
        .layer(oidc_auth_service) // provides (optional) oidc claims
        .layer(session_layer)
        .route("/", get(root))
        .route("/scoreboard", get(get_scores))
        .route("/user/stats", post(post_stats))
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

async fn create_db_pool() -> anyhow::Result<SqlitePool> {
    if !Sqlite::database_exists(DB_PATH).await? {
        Sqlite::create_database(DB_PATH).await?;
    }
    let db_pool = SqlitePool::connect(DB_PATH).await?;
    let mut db = db_pool.acquire().await?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                steps INTEGER,
                seconds INTEGER,
                dances INTEGER
            );",
    )
    .await?;

    Ok(db_pool)
}

async fn authenticated(
    extract::State(state): extract::State<AppState>,
    claims: OidcClaims<AdditionalClaims>,
) -> impl IntoResponse {
    let subject = claims.subject().as_str();
    let rec = sqlx::query!("SELECT id FROM users WHERE oidc_subject = $1", subject)
        .fetch_one(&state.pg_db_pool)
        .await
        .unwrap();

    format!("Hello {}, you are user number {}", subject, rec.id)
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
