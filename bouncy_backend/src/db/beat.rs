use crate::{combo::ComboId, AppState};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct BeatId(i64);

#[derive(Clone, Debug)]
pub struct Beat {
    pub id: BeatId,
    pub start: i32,
    pub duration: i32,
    pub bpm: f32,
    pub subbeat_per_move: i16,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub(crate) struct BeatRow {
    id: i64,
    start: i32,
    duration: i32,
    bpm: f32,
    subbeat_per_move: i16,
}

impl BeatId {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl Beat {
    pub async fn create_for_combo(
        state: &AppState,
        combo_id: ComboId,
        start: i32,
        duration: i32,
        bpm: f32,
        subbeat_per_move: i16,
    ) -> Result<Beat, sqlx::Error> {
        let row = sqlx::query_as!(
            BeatRow,
            r#"
            INSERT INTO video_beats (start, duration, bpm, subbeat_per_move)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                start,
                duration,
                bpm,
                subbeat_per_move
            "#,
            start,
            duration,
            bpm,
            subbeat_per_move
        )
        .fetch_one(&state.pg_db_pool)
        .await?;
        let beat = Beat::from(row);

        let _row = sqlx::query!(
            r#"
            INSERT INTO combos_video_beats (
                combo_id,
                video_beat_id
            )
            VALUES ($1, $2)
            "#,
            combo_id.num(),
            beat.id.num()
        )
        .execute(&state.pg_db_pool)
        .await?;

        Ok(beat)
    }

    pub async fn list_by_combo(
        state: &AppState,
        combo_id: ComboId,
    ) -> Result<Vec<Beat>, sqlx::Error> {
        let rows = sqlx::query_as!(
            BeatRow,
            r#"
            SELECT
                id,
                start,
                duration,
                bpm,
                subbeat_per_move
            FROM video_beats t
                JOIN combos_video_beats ct ON ct.video_beat_id = t.id
            WHERE ct.combo_id = $1
            "#,
            combo_id.num()
        )
        .fetch_all(&state.pg_db_pool)
        .await?;

        let beats = rows.into_iter().map(Beat::from).collect();
        Ok(beats)
    }

    pub async fn delete(state: &AppState, beat_id: BeatId) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_as!(
            BeatRow,
            r#"DELETE FROM video_beats t WHERE t.id = $1"#,
            beat_id.num()
        )
        .execute(&state.pg_db_pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }
}

impl From<BeatRow> for Beat {
    fn from(other: BeatRow) -> Beat {
        Beat {
            id: BeatId(other.id),
            start: other.start,
            duration: other.duration,
            bpm: other.bpm,
            subbeat_per_move: other.subbeat_per_move,
        }
    }
}
