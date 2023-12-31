use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{extract, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteRow;
use sqlx::{Executor, Sqlite, SqlitePool};
use tokio::net::TcpListener;

const DB_PATH: &str = "sqlite:data/db.sqlite";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let create_db_pool = create_db_pool().await?;
    let app = Router::new()
        .route("/", get(root))
        .route("/scoreboard", get(get_scores))
        .route("/user/stats", post(post_stats))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(create_db_pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Deserialize)]
struct NewUserStats {
    id: String,
    name: String,
    steps: u64,
    seconds: u64,
    dances: u64,
}

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: String,
    name: String,
    steps: i64,
    seconds: i64,
    dances: i64,
}

#[derive(Serialize)]
struct UserScore {
    name: String,
    steps: u64,
}

async fn root() -> String {
    format!(
        "Bouncy Feet stats API server running.\nPackage version: {}",
        env!("CARGO_PKG_VERSION")
    )
}
async fn get_scores(
    extract::State(db_pool): extract::State<SqlitePool>,
) -> Result<Json<Vec<UserScore>>, (StatusCode, String)> {
    let users: Vec<User> =
        sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY steps DESC LIMIT 100")
            .fetch_all(&db_pool)
            .await
            .map_err(internal_error)?;
    let scores = users
        .into_iter()
        .map(|u| UserScore {
            name: u.name,
            steps: u.steps as u64,
        })
        .collect();
    Ok(Json(scores))
}
async fn post_stats(
    extract::State(db_pool): extract::State<SqlitePool>,
    extract::Json(user): extract::Json<NewUserStats>,
) -> Result<(), (StatusCode, String)> {
    let mut tx = db_pool.begin().await.map_err(internal_error)?;

    let db_formatted_user = User {
        id: user.id,
        name: user.name,
        steps: user.steps as i64,
        seconds: user.seconds as i64,
        dances: user.dances as i64,
    };
    match insert_or_update_user(&mut tx, &db_formatted_user).await {
        Ok(()) => {
            tx.commit().await.map_err(internal_error)?;
            Ok(())
        }
        Err(e) => {
            tx.rollback().await.map_err(internal_error)?;
            Err(internal_error(e))
        }
    }
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

async fn insert_or_update_user(
    tx: &mut sqlx::Transaction<'_, Sqlite>,
    user: &User,
) -> Result<(), sqlx::Error> {
    let existing_user: Option<SqliteRow> = tx
        .fetch_optional(sqlx::query("SELECT * FROM users WHERE id = ?").bind(&user.id))
        .await?;

    if existing_user.is_some() {
        // Update the existing user
        tx.execute(
            sqlx::query(
                "UPDATE users SET name = ?, steps = ?, seconds = ?, dances = ? WHERE id = ?",
            )
            .bind(&user.name)
            .bind(user.steps)
            .bind(user.seconds)
            .bind(user.dances)
            .bind(&user.id),
        )
        .await?;
    } else {
        // Insert a new user
        tx.execute(
            sqlx::query(
                "INSERT INTO users (id, name, steps, seconds, dances) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&user.id)
            .bind(&user.name)
            .bind(user.steps)
            .bind(user.seconds)
            .bind(user.dances),
        )
        .await?;
    }

    Ok(())
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
