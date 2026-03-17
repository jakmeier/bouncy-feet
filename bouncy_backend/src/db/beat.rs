use crate::{
    api_endoints::combo::NewBeatInfo, db_err_to_status, AppState, CheckedBeatId, CheckedComboId,
};
use axum::http::StatusCode;

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
    pub pose_file: Option<String>,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub(crate) struct BeatRow {
    id: i64,
    start: i32,
    duration: i32,
    bpm: f32,
    subbeat_per_move: i16,
    pose_file: Option<String>,
}

impl BeatId {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl Beat {
    pub async fn create_for_combo(
        state: &AppState,
        checked_combo_id: CheckedComboId,
        data: NewBeatInfo,
    ) -> Result<Beat, (StatusCode, &'static str)> {
        let combo_id = checked_combo_id.assert_write_access()?;

        let row = sqlx::query_as!(
            BeatRow,
            r#"
            INSERT INTO video_beats (start, duration, bpm, subbeat_per_move, pose_file)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                start,
                duration,
                bpm,
                subbeat_per_move,
                pose_file
            "#,
            data.start,
            data.duration,
            data.bpm,
            data.subbeat_per_move,
            data.pose_file
        )
        .fetch_one(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;

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
        .await
        .map_err(db_err_to_status)?;

        Ok(beat)
    }

    pub async fn list_by_combo(
        state: &AppState,
        checked_combo_id: CheckedComboId,
    ) -> Result<Vec<Beat>, (StatusCode, &'static str)> {
        let combo_id = checked_combo_id.assert_read_access()?;

        let rows = sqlx::query_as!(
            BeatRow,
            r#"
            SELECT
                id,
                start,
                duration,
                bpm,
                subbeat_per_move,
                pose_file
            FROM video_beats t
                JOIN combos_video_beats ct ON ct.video_beat_id = t.id
            WHERE ct.combo_id = $1
            "#,
            combo_id.num()
        )
        .fetch_all(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;

        let beats = rows.into_iter().map(Beat::from).collect();
        Ok(beats)
    }

    pub async fn delete(
        state: &AppState,
        checked_beat_id: CheckedBeatId,
    ) -> Result<bool, (StatusCode, &'static str)> {
        let beat_id = checked_beat_id.assert_write_access()?;

        let res = sqlx::query_as!(
            BeatRow,
            r#"DELETE FROM video_beats t WHERE t.id = $1"#,
            beat_id.num()
        )
        .execute(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;
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
            pose_file: other.pose_file,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combo::ComboId;
    use crate::db::test_helpers::make_test_state;
    use sqlx::PgPool;

    /// Insert a guest user and one combo; return the raw ComboId.
    async fn setup_combo(pool: &PgPool) -> ComboId {
        let user_id: i64 =
            sqlx::query_scalar("INSERT INTO users (oidc_subject) VALUES (null) RETURNING id")
                .fetch_one(pool)
                .await
                .expect("failed to insert test user");

        let combo_id: i64 = sqlx::query_scalar(
            "INSERT INTO combos (user_id, is_private) VALUES ($1, false) RETURNING id",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .expect("failed to insert test combo");

        ComboId::for_test(combo_id)
    }

    fn new_beat_input() -> NewBeatInfo {
        NewBeatInfo {
            start: 250,
            duration: 4000,
            bpm: 120.0,
            subbeat_per_move: 2,
            pose_file: Some("dGVzdAo=".to_owned()),
        }
    }

    // ── Beat::create_for_combo ────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn create_beat_for_owned_combo_returns_correct_fields(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;
        let checked = CheckedComboId::Owned(combo_id);

        let new_input = NewBeatInfo {
            start: 100,
            duration: 480,
            bpm: 132.0,
            subbeat_per_move: 1,
            pose_file: Some("dGVzdAo=".to_owned()),
        };
        let beat = Beat::create_for_combo(&state, checked, new_input.clone())
            .await
            .expect("create_for_combo should succeed for an owned combo");

        assert_eq!(beat.start, new_input.start);
        assert_eq!(beat.duration, new_input.duration);
        assert_eq!(beat.bpm, new_input.bpm);
        assert_eq!(beat.subbeat_per_move, new_input.subbeat_per_move);
        assert!(beat.id.num() > 0, "assigned id should be positive");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn create_beat_for_public_combo_is_forbidden(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;
        // Public means readable but not writable.
        let checked = CheckedComboId::Public(combo_id);

        let result = Beat::create_for_combo(&state, checked, new_beat_input()).await;

        assert!(
            result.is_err(),
            "write to a public (non-owned) combo must be rejected"
        );
        let (status, _) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::FORBIDDEN);
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn create_beat_for_not_found_combo_returns_404(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool);
        let checked = CheckedComboId::NotFound;

        let result = Beat::create_for_combo(&state, checked, new_beat_input()).await;

        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::NOT_FOUND);
        Ok(())
    }

    // ── Beat::list_by_combo ───────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_by_combo_returns_inserted_beats(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;
        let owned = CheckedComboId::Owned(combo_id);

        Beat::create_for_combo(&state, owned, new_beat_input())
            .await
            .unwrap();
        Beat::create_for_combo(&state, owned, new_beat_input())
            .await
            .unwrap();

        let beats = Beat::list_by_combo(&state, owned)
            .await
            .expect("list should succeed");
        assert_eq!(beats.len(), 2);
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_by_combo_is_isolated_per_combo(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_a = setup_combo(&pool).await;
        let combo_b = setup_combo(&pool).await;

        Beat::create_for_combo(&state, CheckedComboId::Owned(combo_a), new_beat_input())
            .await
            .unwrap();
        Beat::create_for_combo(&state, CheckedComboId::Owned(combo_b), new_beat_input())
            .await
            .unwrap();
        Beat::create_for_combo(&state, CheckedComboId::Owned(combo_b), new_beat_input())
            .await
            .unwrap();

        let beats_a = Beat::list_by_combo(&state, CheckedComboId::Owned(combo_a))
            .await
            .unwrap();
        let beats_b = Beat::list_by_combo(&state, CheckedComboId::Owned(combo_b))
            .await
            .unwrap();

        assert_eq!(beats_a.len(), 1, "combo_a should have exactly 1 beat");
        assert_eq!(beats_b.len(), 2, "combo_b should have exactly 2 beats");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_by_combo_empty_for_new_combo(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        let beats = Beat::list_by_combo(&state, CheckedComboId::Owned(combo_id))
            .await
            .expect("list should succeed");
        assert!(beats.is_empty(), "new combo should have no beats");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_by_public_combo_is_allowed(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        // A public (non-owned) combo should still be readable.
        let beats = Beat::list_by_combo(&state, CheckedComboId::Public(combo_id))
            .await
            .expect("read access on a public combo must succeed");
        assert!(beats.is_empty());
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_by_combo_not_found_returns_404(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool);

        let result = Beat::list_by_combo(&state, CheckedComboId::NotFound).await;

        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::NOT_FOUND);
        Ok(())
    }

    // ── Beat::delete ──────────────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_owned_beat_returns_true(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        let beat =
            Beat::create_for_combo(&state, CheckedComboId::Owned(combo_id), new_beat_input())
                .await
                .unwrap();

        let deleted = Beat::delete(&state, CheckedBeatId::Owned(beat.id))
            .await
            .expect("delete should succeed for an owned beat");
        assert!(deleted, "delete should return true when the beat existed");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_removes_beat_from_list(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        let beat =
            Beat::create_for_combo(&state, CheckedComboId::Owned(combo_id), new_beat_input())
                .await
                .unwrap();
        Beat::delete(&state, CheckedBeatId::Owned(beat.id))
            .await
            .unwrap();

        let beats = Beat::list_by_combo(&state, CheckedComboId::Owned(combo_id))
            .await
            .unwrap();
        assert!(
            beats.is_empty(),
            "beat should no longer appear in the list after deletion"
        );
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_nonexistent_beat_returns_false(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool);

        let deleted = Beat::delete(&state, CheckedBeatId::Owned(BeatId(i64::MAX)))
            .await
            .expect("delete of non-existent beat should not error");
        assert!(
            !deleted,
            "delete should return false when the beat did not exist"
        );
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_public_beat_is_forbidden(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let combo_id = setup_combo(&pool).await;

        let beat =
            Beat::create_for_combo(&state, CheckedComboId::Owned(combo_id), new_beat_input())
                .await
                .unwrap();

        // Public means read-only; deleting should be rejected.
        let result = Beat::delete(&state, CheckedBeatId::Public(beat.id)).await;

        assert!(result.is_err(), "deleting a public beat must be rejected");
        let (status, _) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::FORBIDDEN);
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_not_found_beat_returns_404(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool);

        let result = Beat::delete(&state, CheckedBeatId::NotFound).await;

        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::NOT_FOUND);
        Ok(())
    }

    // ── Pure unit tests (no DB) ───────────────────────────────────────────────

    #[test]
    fn beat_id_num_roundtrips() {
        let id = BeatId(42);
        assert_eq!(id.num(), 42);
    }

    #[test]
    fn beat_from_row_maps_all_fields() {
        let row = BeatRow {
            id: 7,
            start: 100,
            duration: 500,
            bpm: 128.5,
            subbeat_per_move: 3,
            pose_file: Some("should be base64 but isn't".to_owned()),
        };
        let beat = Beat::from(row);
        assert_eq!(beat.id.num(), 7);
        assert_eq!(beat.start, 100);
        assert_eq!(beat.duration, 500);
        assert_eq!((beat.bpm * 10.0).round(), 1285.0);
        assert_eq!(beat.subbeat_per_move, 3);
        assert_eq!(
            beat.pose_file,
            Some("should be base64 but isn't".to_owned())
        );
    }
}
