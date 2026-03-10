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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    /// Insert a guest user and a combo, return the combo_id for use in beat tests.
    async fn setup_combo(pool: &PgPool) -> ComboId {
        let user_id = sqlx::query!(
            "INSERT INTO users (oidc_subject) VALUES (null) RETURNING id"
        )
        .fetch_one(pool)
        .await
        .expect("failed to insert test user")
        .id;

        let combo_id = sqlx::query!(
            r#"
            INSERT INTO combos (user_id, is_private)
            VALUES ($1, false)
            RETURNING id
            "#,
            user_id
        )
        .fetch_one(pool)
        .await
        .expect("failed to insert test combo")
        .id;

        ComboId(combo_id)
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn create_beat_for_combo_returns_correct_fields(pool: PgPool) -> sqlx::Result<()> {
        let state = crate::make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        let beat = Beat::create_for_combo(&state, combo_id, 100, 480, 125.0, 4).await?;

        assert_eq!(beat.start, 100);
        assert_eq!(beat.duration, 480);
        assert_eq!((beat.bpm * 10.0).round(), 1250.0);
        assert_eq!(beat.subbeat_per_move, 4);
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_by_combo_returns_inserted_beats(pool: PgPool) -> sqlx::Result<()> {
        let state = crate::make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        Beat::create_for_combo(&state, combo_id, 0, 300, 120.0, 2).await?;
        Beat::create_for_combo(&state, combo_id, 300, 600, 130.0, 4).await?;

        let beats = Beat::list_by_combo(&state, combo_id).await?;

        assert_eq!(beats.len(), 2);
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_by_combo_returns_only_beats_for_that_combo(pool: PgPool) -> sqlx::Result<()> {
        let state = crate::make_test_state(pool.clone());
        let combo_a = setup_combo(&pool).await;
        let combo_b = setup_combo(&pool).await;

        Beat::create_for_combo(&state, combo_a, 0, 480, 120.0, 4).await?;
        Beat::create_for_combo(&state, combo_b, 0, 200, 100.0, 2).await?;
        Beat::create_for_combo(&state, combo_b, 200, 400, 110.0, 2).await?;

        let beats_a = Beat::list_by_combo(&state, combo_a).await?;
        let beats_b = Beat::list_by_combo(&state, combo_b).await?;

        assert_eq!(beats_a.len(), 1, "combo_a should have exactly 1 beat");
        assert_eq!(beats_b.len(), 2, "combo_b should have exactly 2 beats");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_by_combo_empty_for_new_combo(pool: PgPool) -> sqlx::Result<()> {
        let state = crate::make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        let beats = Beat::list_by_combo(&state, combo_id).await?;

        assert!(beats.is_empty(), "new combo should have no beats");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_existing_beat_returns_true(pool: PgPool) -> sqlx::Result<()> {
        let state = crate::make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        let beat = Beat::create_for_combo(&state, combo_id, 0, 480, 120.0, 4).await?;
        let deleted = Beat::delete(&state, beat.id).await?;

        assert!(deleted, "delete should return true for an existing beat");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_removes_beat_from_list(pool: PgPool) -> sqlx::Result<()> {
        let state = crate::make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        let beat = Beat::create_for_combo(&state, combo_id, 0, 480, 120.0, 4).await?;
        Beat::delete(&state, beat.id).await?;

        let beats = Beat::list_by_combo(&state, combo_id).await?;
        assert!(beats.is_empty(), "list should be empty after deleting the only beat");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_nonexistent_beat_returns_false(pool: PgPool) -> sqlx::Result<()> {
        let state = crate::make_test_state(pool.clone());

        let nonexistent_id = BeatId(i64::MAX);
        let deleted = Beat::delete(&state, nonexistent_id).await?;

        assert!(!deleted, "delete should return false for a beat that doesn't exist");
        Ok(())
    }

    #[test]
    fn beat_id_num_roundtrips() {
        let id = BeatId(42);
        assert_eq!(id.num(), 42);
    }

    #[test]
    fn beat_from_row_sets_all_fields() {
        let row = BeatRow {
            id: 7,
            start: 100,
            duration: 500,
            bpm: 128.5,
            subbeat_per_move: 3,
        };
        let beat = Beat::from(row);
        assert_eq!(beat.id.num(), 7);
        assert_eq!(beat.start, 100);
        assert_eq!(beat.duration, 500);
        assert_eq!((beat.bpm * 10.0).round(), 1285.0);
        assert_eq!(beat.subbeat_per_move, 3);
    }
}
