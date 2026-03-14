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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::apply_migrations;
    use crate::user::UserId;
    use uuid::Uuid;

    /// Insert a guest user and create a client session; return the session ID.
    async fn setup_client_session(pool: &PgPool) -> i64 {
        let user_id = UserId::create_new_guest(pool).await;
        let secret = Uuid::new_v4();

        let session_id: i64 = sqlx::query_scalar(
            "INSERT INTO client_session (user_id, client_session_secret) VALUES ($1, $2) RETURNING id",
        )
        .bind(user_id.num())
        .bind(secret)
        .fetch_one(pool)
        .await
        .expect("failed to insert test client session");

        session_id
    }

    /// Create a `NewDanceActivity` with the given parameters and a fixed timestamp.
    fn make_activity(
        client_session_id: i64,
        activity_id: &str,
        hits: i32,
        misses: i32,
        steps: i32,
    ) -> NewDanceActivity {
        let recorded_at = NaiveDateTime::parse_from_str("2024-06-01 12:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("valid timestamp");
        NewDanceActivity {
            client_session_id,
            activity_id: activity_id.to_string(),
            recorded_at,
            hits,
            misses,
            steps,
        }
    }

    // ── NewDanceActivity::store_to_db ─────────────────────────────────────────

    #[sqlx::test]
    async fn store_valid_activity_succeeds(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        let activity = make_activity(session_id, "lesson:step-a:120bpm", 10, 2, 5);
        let result = activity.store_to_db(&pool).await;

        assert!(result.is_ok(), "store_to_db should succeed for a valid activity");
        Ok(())
    }

    #[sqlx::test]
    async fn store_activity_persists_all_fields(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        let timestamp =
            NaiveDateTime::parse_from_str("2024-03-15 09:30:00", "%Y-%m-%d %H:%M:%S")
                .expect("valid timestamp");
        let activity = NewDanceActivity {
            client_session_id: session_id,
            activity_id: "lesson:waltz:100bpm".to_string(),
            recorded_at: timestamp,
            hits: 15,
            misses: 3,
            steps: 8,
        };
        activity.store_to_db(&pool).await.expect("insert failed");

        #[derive(sqlx::FromRow)]
        struct Row {
            client_session_id: i64,
            recorded_at: NaiveDateTime,
            activity_id: String,
            hits: i32,
            misses: i32,
            steps: i32,
        }

        let row: Row = sqlx::query_as(
            "SELECT client_session_id, recorded_at, activity_id, hits, misses, steps \
             FROM dance_activity \
             WHERE activity_id = $1",
        )
        .bind("lesson:waltz:100bpm")
        .fetch_one(&pool)
        .await?;

        assert_eq!(row.client_session_id, session_id, "client_session_id must match");
        assert_eq!(row.recorded_at, timestamp, "recorded_at must be preserved exactly");
        assert_eq!(row.activity_id, "lesson:waltz:100bpm", "activity_id must match");
        assert_eq!(row.hits, 15, "hits must match");
        assert_eq!(row.misses, 3, "misses must match");
        assert_eq!(row.steps, 8, "steps must match");
        Ok(())
    }

    #[sqlx::test]
    async fn store_activity_with_zero_stats_succeeds(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        let activity = make_activity(session_id, "lesson:empty", 0, 0, 0);
        let result = activity.store_to_db(&pool).await;

        assert!(result.is_ok(), "zero hits/misses/steps should be accepted by the DB CHECK constraint");

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM dance_activity WHERE activity_id = 'lesson:empty'",
        )
        .fetch_one(&pool)
        .await?;
        assert_eq!(count, 1, "zero-stats activity should appear in the table");
        Ok(())
    }

    #[sqlx::test]
    async fn store_activity_with_large_stats_succeeds(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        let activity = make_activity(session_id, "high-volume", i32::MAX, i32::MAX, i32::MAX);
        let result = activity.store_to_db(&pool).await;

        assert!(result.is_ok(), "large integer stats should be stored successfully");
        Ok(())
    }

    #[sqlx::test]
    async fn store_multiple_activities_for_same_session(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        for i in 0..3_i32 {
            let act = make_activity(session_id, &format!("activity_{i}"), i * 10, i, i * 5);
            act.store_to_db(&pool).await.expect("insert failed");
        }

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM dance_activity WHERE client_session_id = $1",
        )
        .bind(session_id)
        .fetch_one(&pool)
        .await?;

        assert_eq!(count, 3, "all 3 activities for the session should be stored");
        Ok(())
    }

    #[sqlx::test]
    async fn activities_are_isolated_per_session(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_a = setup_client_session(&pool).await;
        let session_b = setup_client_session(&pool).await;

        make_activity(session_a, "act-a1", 10, 1, 5)
            .store_to_db(&pool)
            .await
            .unwrap();
        make_activity(session_b, "act-b1", 20, 2, 10)
            .store_to_db(&pool)
            .await
            .unwrap();
        make_activity(session_b, "act-b2", 30, 3, 15)
            .store_to_db(&pool)
            .await
            .unwrap();

        let count_a: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM dance_activity WHERE client_session_id = $1")
                .bind(session_a)
                .fetch_one(&pool)
                .await?;
        let count_b: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM dance_activity WHERE client_session_id = $1")
                .bind(session_b)
                .fetch_one(&pool)
                .await?;

        assert_eq!(count_a, 1, "session A should have exactly 1 activity");
        assert_eq!(count_b, 2, "session B should have exactly 2 activities");
        Ok(())
    }

    #[sqlx::test]
    async fn store_activity_with_special_chars_in_activity_id(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        let id = "lesson:step-a_v2.0@120bpm";
        let activity = make_activity(session_id, id, 5, 0, 3);
        activity.store_to_db(&pool).await.expect("insert failed");

        let stored: String =
            sqlx::query_scalar("SELECT activity_id FROM dance_activity WHERE activity_id = $1")
                .bind(id)
                .fetch_one(&pool)
                .await?;

        assert_eq!(stored, id, "activity_id with special characters should round-trip exactly");
        Ok(())
    }

    #[sqlx::test]
    async fn store_activity_with_max_length_activity_id_succeeds(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        // The schema defines activity_id as VARCHAR(256); a 256-character value must succeed.
        let max_id = "x".repeat(256);
        let activity = make_activity(session_id, &max_id, 1, 0, 1);
        let result = activity.store_to_db(&pool).await;

        assert!(result.is_ok(), "activity_id at max VARCHAR(256) length should be accepted");
        Ok(())
    }

    #[sqlx::test]
    async fn store_activity_with_overlong_activity_id_fails(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        // 257 characters exceeds the VARCHAR(256) limit.
        let too_long = "x".repeat(257);
        let activity = make_activity(session_id, &too_long, 1, 0, 1);
        let result = activity.store_to_db(&pool).await;

        assert!(
            result.is_err(),
            "activity_id exceeding VARCHAR(256) should be rejected by the database"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn store_activity_fails_for_nonexistent_session(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;

        // i64::MAX is guaranteed not to exist in a fresh DB.
        let activity = make_activity(i64::MAX, "orphan-activity", 5, 0, 3);
        let result = activity.store_to_db(&pool).await;

        assert!(
            result.is_err(),
            "inserting with a non-existent client_session_id should violate the FK constraint"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn store_activity_preserves_recorded_at_timestamp(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        let timestamp =
            NaiveDateTime::parse_from_str("2023-12-25 23:59:59", "%Y-%m-%d %H:%M:%S")
                .expect("valid timestamp");
        let activity = NewDanceActivity {
            client_session_id: session_id,
            activity_id: "christmas-dance".to_string(),
            recorded_at: timestamp,
            hits: 7,
            misses: 1,
            steps: 4,
        };
        activity.store_to_db(&pool).await.expect("insert failed");

        let stored_ts: NaiveDateTime = sqlx::query_scalar(
            "SELECT recorded_at FROM dance_activity WHERE activity_id = 'christmas-dance'",
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(stored_ts, timestamp, "recorded_at should survive a DB round-trip unchanged");
        Ok(())
    }

    #[sqlx::test]
    async fn store_same_activity_id_twice_for_same_session_succeeds(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        // The schema has no UNIQUE constraint on (client_session_id, activity_id), so
        // duplicate activity_ids for the same session are permitted (though unusual).
        let act1 = make_activity(session_id, "repeated-activity", 10, 1, 5);
        let act2 = make_activity(session_id, "repeated-activity", 20, 2, 10);

        act1.store_to_db(&pool).await.expect("first insert failed");
        act2.store_to_db(&pool).await.expect("second insert failed");

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM dance_activity WHERE activity_id = 'repeated-activity'",
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(count, 2, "the same activity_id can appear multiple times for a session");
        Ok(())
    }

    #[sqlx::test]
    async fn concurrent_inserts_all_succeed(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let session_id = setup_client_session(&pool).await;

        let handles: Vec<_> = (0..5_i32)
            .map(|i| {
                let pool = pool.clone();
                let activity = make_activity(session_id, &format!("concurrent-{i}"), i, 0, i);
                tokio::spawn(async move { activity.store_to_db(&pool).await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.expect("task panicked");
            assert!(result.is_ok(), "each concurrent insert should succeed");
        }

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM dance_activity WHERE client_session_id = $1",
        )
        .bind(session_id)
        .fetch_one(&pool)
        .await?;

        assert_eq!(count, 5, "all 5 concurrent inserts should have been committed");
        Ok(())
    }

    // ── Pure unit tests (no DB) ───────────────────────────────────────────────

    #[test]
    fn new_dance_activity_fields_are_accessible() {
        let ts = NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("valid timestamp");
        let activity = NewDanceActivity {
            client_session_id: 42,
            activity_id: "unit-test".to_string(),
            recorded_at: ts,
            hits: 50,
            misses: 5,
            steps: 25,
        };

        assert_eq!(activity.client_session_id, 42);
        assert_eq!(activity.activity_id, "unit-test");
        assert_eq!(activity.recorded_at, ts);
        assert_eq!(activity.hits, 50);
        assert_eq!(activity.misses, 5);
        assert_eq!(activity.steps, 25);
    }

    #[test]
    fn make_activity_helper_sets_correct_values() {
        let activity = make_activity(7, "helper-test", 20, 3, 10);

        assert_eq!(activity.client_session_id, 7);
        assert_eq!(activity.activity_id, "helper-test");
        assert_eq!(activity.hits, 20);
        assert_eq!(activity.misses, 3);
        assert_eq!(activity.steps, 10);
    }
}
