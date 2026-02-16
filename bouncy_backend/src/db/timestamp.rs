use crate::{combo::ComboId, AppState};

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
        combo_id: ComboId,
        milliseconds: i32,
    ) -> Result<Timestamp, sqlx::Error> {
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
        .await?;
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
        .await?;

        Ok(timestamp)
    }

    pub async fn list_by_combo(
        state: &AppState,
        combo_id: ComboId,
    ) -> Result<Vec<Timestamp>, sqlx::Error> {
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
        .await?;

        let timestamps = rows.into_iter().map(Timestamp::from).collect();
        Ok(timestamps)
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
