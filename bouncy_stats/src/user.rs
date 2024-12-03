use crate::{internal_error, AppState};
use axum::http::StatusCode;
use axum::{extract, Json};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Executor, Sqlite};

#[derive(Deserialize)]
pub(crate) struct NewUserStats {
    id: String,
    name: String,
    steps: u64,
    seconds: u64,
    dances: u64,
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct User {
    id: String,
    name: String,
    steps: i64,
    seconds: i64,
    dances: i64,
}

#[derive(Serialize)]
pub(crate) struct UserScore {
    name: String,
    steps: u64,
}

pub(crate) async fn get_scores(
    extract::State(state): extract::State<AppState>,
) -> Result<Json<Vec<UserScore>>, (StatusCode, String)> {
    let db_pool = &state.db_pool;
    let users: Vec<User> =
        sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY steps DESC LIMIT 100")
            .fetch_all(db_pool)
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

pub(crate) async fn post_stats(
    extract::State(state): extract::State<AppState>,
    extract::Json(user): extract::Json<NewUserStats>,
) -> Result<(), (StatusCode, String)> {
    let db_pool = &state.db_pool;
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

pub(crate) async fn insert_or_update_user(
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
