use crate::{db_err_to_status, AppState, CheckedComboId, CheckedTimestampId};
use axum::http::StatusCode;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct TimestampId(i64);

#[derive(Clone, Debug)]
pub struct Timestamp {
    pub id: TimestampId,
    pub milliseconds: i32,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub(crate) struct TimestampRow {
    id: i64,
    milliseconds: i32,
}

impl TimestampId {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl Timestamp {
    pub async fn create_for_combo(
        state: &AppState,
        checked_combo_id: CheckedComboId,
        milliseconds: i32,
    ) -> Result<Timestamp, (StatusCode, &'static str)> {
        let combo_id = checked_combo_id.assert_write_access()?;
        let row = sqlx::query_as!(
            TimestampRow,
            r#"
            INSERT INTO video_timestamps (milliseconds)
            VALUES ($1)
            RETURNING
                id,
                milliseconds
            "#,
            milliseconds
        )
        .fetch_one(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;
        let timestamp = Timestamp::from(row);

        let _row = sqlx::query!(
            r#"
            INSERT INTO combos_video_timestamps (
                combo_id,
                video_timestamp_id
            )
            VALUES ($1, $2)
            "#,
            combo_id.num(),
            timestamp.id.num()
        )
        .execute(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;

        Ok(timestamp)
    }

    pub async fn list_by_combo(
        state: &AppState,
        checked_combo_id: CheckedComboId,
    ) -> Result<Vec<Timestamp>, (StatusCode, &'static str)> {
        let combo_id = checked_combo_id.assert_read_access()?;

        let rows = sqlx::query_as!(
            TimestampRow,
            r#"
            SELECT
                id,
                milliseconds
            FROM video_timestamps t
                JOIN combos_video_timestamps ct ON ct.video_timestamp_id = t.id
            WHERE ct.combo_id = $1
            "#,
            combo_id.num()
        )
        .fetch_all(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;

        let timestamps = rows.into_iter().map(Timestamp::from).collect();
        Ok(timestamps)
    }

    pub async fn delete(
        state: &AppState,
        checked_timestamp_id: CheckedTimestampId,
    ) -> Result<bool, (StatusCode, &'static str)> {
        let timestamp_id = checked_timestamp_id.assert_write_access()?;
        let res = sqlx::query_as!(
            TimestampRow,
            r#"DELETE FROM video_timestamps t WHERE t.id = $1"#,
            timestamp_id.num()
        )
        .execute(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;
        Ok(res.rows_affected() > 0)
    }
}

impl From<TimestampRow> for Timestamp {
    fn from(other: TimestampRow) -> Timestamp {
        Timestamp {
            id: TimestampId(other.id),
            milliseconds: other.milliseconds,
        }
    }
}
