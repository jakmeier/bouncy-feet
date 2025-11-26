use anyhow::bail;
use chrono::NaiveDateTime;
use sqlx::PgPool;

// #[derive(Debug, sqlx::FromRow)]
// pub(crate) struct DanceActivity {
//     pub dance_activity_id: i64,
//     pub client_session_id: i64,
//     pub recorded_at: NaiveDateTime,
//     pub activity_id: String,
//     pub hits: i32,
//     pub misses: i32,
//     pub steps: i32,
// }

pub(crate) struct NewDanceActivity {
    pub client_session_id: i64,
    pub activity_id: String,
    pub recorded_at: NaiveDateTime,
    pub hits: i32,
    pub misses: i32,
    pub steps: i32,
}

impl NewDanceActivity {
    pub(crate) async fn store_to_db(&self, db_pool: &PgPool) -> anyhow::Result<()> {
        // TODO: probably the API server should only accept newer recordings than the latest, per client session

        let result = sqlx::query!(
            r#"
            INSERT INTO dance_activity (client_session_id, recorded_at, activity_id, hits, misses, steps)
            VALUES($1, $2, $3, $4, $5, $6)
            "#,
            self.client_session_id, self.recorded_at, self.activity_id, self.hits, self.misses, self.steps
        )
        .execute(db_pool)
        .await;

        match result {
            Ok(query_result) => {
                if query_result.rows_affected() == 1 {
                    Ok(())
                } else {
                    bail!("inserting failed")
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
