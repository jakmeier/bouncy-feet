use crate::{
    api_endoints::combo::ComboInfo,
    beat::BeatId,
    timestamp::TimestampId,
    user::{User, UserId},
    AppState,
};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ComboId(i64);

#[derive(Clone, Debug)]
pub struct Combo {
    pub id: ComboId,
    #[allow(unused)]
    pub user_id: UserId,
    pub is_private: bool,
    pub sort_order: Option<i32>,
    pub free_form_category: Option<String>,
    pub title: Option<String>,
    pub video_short_uuid: Option<String>,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub(crate) struct ComboRow {
    id: i64,
    pub user_id: i64,
    is_private: bool,
    sort_order: Option<i32>,
    free_form_category: Option<String>,
    title: Option<String>,
    video_short_uuid: Option<String>,
}

impl ComboId {
    pub fn num(&self) -> i64 {
        self.0
    }

    /// Construct a `ComboId` from a raw database id. Only available in test
    /// code; production code must obtain IDs through the normal DB functions.
    #[cfg(test)]
    pub(crate) fn for_test(id: i64) -> Self {
        ComboId(id)
    }
}

impl Combo {
    pub async fn create(
        state: &AppState,
        user_id: UserId,
        is_private: bool,
        sort_order: Option<i32>,
        free_form_category: Option<&str>,
        title: Option<&str>,
        video_short_uuid: Option<&str>,
    ) -> Result<Combo, sqlx::Error> {
        let row = sqlx::query_as!(
            ComboRow,
            r#"
            INSERT INTO combos (
                user_id,
                is_private,
                sort_order,
                free_form_category,
                title,
                video_short_uuid
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                user_id,
                is_private,
                sort_order,
                free_form_category,
                title,
                video_short_uuid
            "#,
            user_id.num(),
            is_private,
            sort_order,
            free_form_category,
            title,
            video_short_uuid
        )
        .fetch_one(&state.pg_db_pool)
        .await?;

        Ok(Combo::from(row))
    }

    pub async fn lookup(state: &AppState, id: ComboId) -> Result<Option<Combo>, sqlx::Error> {
        let row: Option<ComboRow> = sqlx::query_as!(
            ComboRow,
            r#"
            SELECT
                id,
                user_id,
                is_private,
                sort_order,
                free_form_category,
                title,
                video_short_uuid
            FROM combos
            WHERE id = $1
            "#,
            id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await?;

        Ok(row.map(Combo::from))
    }

    pub async fn is_owned_by(
        state: &AppState,
        id: ComboId,
        user_id: UserId,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM combos
                WHERE id = $1 AND user_id = $2
            )
            "#,
            id.num(),
            user_id.num()
        )
        .fetch_one(&state.pg_db_pool)
        .await
        .map(|maybe| maybe.unwrap_or(false))
    }

    pub async fn list_by_user(
        state: &AppState,
        user_id: UserId,
    ) -> Result<Vec<Combo>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ComboRow,
            r#"
            SELECT
                id,
                user_id,
                is_private,
                sort_order,
                free_form_category,
                title,
                video_short_uuid
            FROM combos
            WHERE user_id = $1
            "#,
            user_id.num()
        )
        .fetch_all(&state.pg_db_pool)
        .await?;

        let combos = rows.into_iter().map(Combo::from).collect();
        Ok(combos)
    }

    pub async fn update_combo(
        state: &AppState,
        current_user: &User,
        payload: ComboInfo,
    ) -> Result<Option<Combo>, sqlx::Error> {
        let row = sqlx::query_as!(
            ComboRow,
            r#"
        UPDATE combos
        SET
            is_private = $3,
            sort_order = $4,
            free_form_category = $5,
            title = $6,
            video_short_uuid = $7
        WHERE id = $1
            AND user_id = $2
        RETURNING
            id,
            user_id,
            is_private,
            sort_order,
            free_form_category,
            title,
            video_short_uuid
        "#,
            payload.id.num(),
            current_user.id.num(),
            payload.is_private,
            payload.sort_order,
            payload.free_form_category.as_deref(),
            payload.title.as_deref(),
            payload.video_short_uuid.as_deref(),
        )
        .fetch_optional(&state.pg_db_pool)
        .await?;

        Ok(row.map(Combo::from))
    }
}

impl From<ComboRow> for Combo {
    fn from(row: ComboRow) -> Self {
        Combo {
            id: ComboId(row.id),
            user_id: row.user_id(),
            is_private: row.is_private,
            sort_order: row.sort_order,
            free_form_category: row.free_form_category,
            title: row.title,
            video_short_uuid: row.video_short_uuid,
        }
    }
}

impl crate::db::timestamp::Timestamp {
    pub async fn combo_of_timestamp(
        state: &AppState,
        timestamp_id: TimestampId,
    ) -> Result<Option<ComboId>, sqlx::Error> {
        let rows = sqlx::query_scalar!(
            r#"
            SELECT
                ct.combo_id
            FROM video_timestamps t
                JOIN combos_video_timestamps ct ON ct.video_timestamp_id = t.id
            WHERE t.id = $1
            "#,
            timestamp_id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await?;

        let id = rows.map(ComboId);
        Ok(id)
    }
}

impl crate::db::beat::Beat {
    pub async fn combo_of_beat(
        state: &AppState,
        beat_id: BeatId,
    ) -> Result<Option<ComboId>, sqlx::Error> {
        let rows = sqlx::query_scalar!(
            r#"
            SELECT
                cb.combo_id
            FROM video_beats b
                JOIN combos_video_beats cb ON cb.video_beat_id = b.id
            WHERE b.id = $1
            "#,
            beat_id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await?;

        let id = rows.map(ComboId);
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::{apply_migrations, make_test_state};
    use sqlx::PgPool;

    /// Insert a guest user; return the UserId.
    async fn setup_user(pool: &PgPool) -> UserId {
        let user_id: i64 = sqlx::query_scalar("INSERT INTO users (oidc_subject) VALUES (null) RETURNING id")
            .fetch_one(pool)
            .await
            .expect("failed to insert test user");
        UserId::from_i64(user_id)
    }

    // ── Combo::create ──────────────────────────────────────────────────────────

    #[sqlx::test]
    async fn create_combo_with_all_fields_returns_correct_values(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let combo = Combo::create(
            &state,
            user_id,
            true,
            Some(1),
            Some("Dance Move"),
            Some("Salsa Turn"),
            Some("short-uuid-abc"),
        )
        .await
        .expect("create should succeed");

        assert!(combo.id.num() > 0, "assigned id should be positive");
        assert_eq!(combo.user_id.num(), user_id.num());
        assert!(combo.is_private);
        assert_eq!(combo.sort_order, Some(1));
        assert_eq!(combo.free_form_category, Some("Dance Move".to_string()));
        assert_eq!(combo.title, Some("Salsa Turn".to_string()));
        assert_eq!(combo.video_short_uuid, Some("short-uuid-abc".to_string()));
        Ok(())
    }

    #[sqlx::test]
    async fn create_combo_with_minimal_fields(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let combo = Combo::create(&state, user_id, false, None, None, None, None)
            .await
            .expect("create should succeed with minimal fields");

        assert!(combo.id.num() > 0);
        assert_eq!(combo.user_id.num(), user_id.num());
        assert!(!combo.is_private);
        assert_eq!(combo.sort_order, None);
        assert_eq!(combo.free_form_category, None);
        assert_eq!(combo.title, None);
        assert_eq!(combo.video_short_uuid, None);
        Ok(())
    }

    #[sqlx::test]
    async fn create_multiple_combos_have_distinct_ids(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let combo1 = Combo::create(&state, user_id, false, None, None, Some("First"), None)
            .await
            .expect("first create should succeed");
        let combo2 = Combo::create(&state, user_id, false, None, None, Some("Second"), None)
            .await
            .expect("second create should succeed");

        assert_ne!(
            combo1.id.num(),
            combo2.id.num(),
            "each combo should have a unique id"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn create_combo_preserves_private_flag(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let private_combo = Combo::create(&state, user_id, true, None, None, None, None)
            .await
            .expect("create private combo");
        let public_combo = Combo::create(&state, user_id, false, None, None, None, None)
            .await
            .expect("create public combo");

        assert!(private_combo.is_private);
        assert!(!public_combo.is_private);
        Ok(())
    }

    // ── Combo::lookup ──────────────────────────────────────────────────────────

    #[sqlx::test]
    async fn lookup_existing_combo_returns_some(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let created = Combo::create(
            &state,
            user_id,
            false,
            Some(5),
            Some("Category"),
            Some("Title"),
            Some("uuid-123"),
        )
        .await
        .expect("create should succeed");

        let looked_up = Combo::lookup(&state, created.id)
            .await
            .expect("lookup should succeed");

        assert!(looked_up.is_some(), "should find the combo that was just created");
        let combo = looked_up.unwrap();
        assert_eq!(combo.id.num(), created.id.num());
        assert_eq!(combo.user_id.num(), user_id.num());
        assert_eq!(combo.sort_order, Some(5));
        assert_eq!(combo.free_form_category, Some("Category".to_string()));
        assert_eq!(combo.title, Some("Title".to_string()));
        assert_eq!(combo.video_short_uuid, Some("uuid-123".to_string()));
        Ok(())
    }

    #[sqlx::test]
    async fn lookup_nonexistent_combo_returns_none(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);

        let result = Combo::lookup(&state, ComboId(i64::MAX))
            .await
            .expect("lookup should not error");

        assert!(result.is_none(), "looking up a non-existent combo should return None");
        Ok(())
    }

    // ── Combo::is_owned_by ─────────────────────────────────────────────────────

    #[sqlx::test]
    async fn is_owned_by_returns_true_for_owner(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let combo = Combo::create(&state, user_id, false, None, None, None, None)
            .await
            .expect("create should succeed");

        let is_owned = Combo::is_owned_by(&state, combo.id, user_id)
            .await
            .expect("is_owned_by should succeed");

        assert!(is_owned, "creator should own the combo");
        Ok(())
    }

    #[sqlx::test]
    async fn is_owned_by_returns_false_for_non_owner(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user1 = setup_user(&state.pg_db_pool).await;
        let user2 = setup_user(&state.pg_db_pool).await;

        let combo = Combo::create(&state, user1, false, None, None, None, None)
            .await
            .expect("create should succeed");

        let is_owned = Combo::is_owned_by(&state, combo.id, user2)
            .await
            .expect("is_owned_by should succeed");

        assert!(!is_owned, "different user should not own the combo");
        Ok(())
    }

    #[sqlx::test]
    async fn is_owned_by_returns_false_for_nonexistent_combo(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let is_owned = Combo::is_owned_by(&state, ComboId(i64::MAX), user_id)
            .await
            .expect("is_owned_by should succeed");

        assert!(!is_owned, "non-existent combo cannot be owned by anyone");
        Ok(())
    }

    // ── Combo::list_by_user ────────────────────────────────────────────────────

    #[sqlx::test]
    async fn list_by_user_returns_all_user_combos(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        Combo::create(&state, user_id, false, None, None, Some("Combo 1"), None)
            .await
            .expect("create combo 1");
        Combo::create(&state, user_id, false, None, None, Some("Combo 2"), None)
            .await
            .expect("create combo 2");
        Combo::create(&state, user_id, false, None, None, Some("Combo 3"), None)
            .await
            .expect("create combo 3");

        let combos = Combo::list_by_user(&state, user_id)
            .await
            .expect("list should succeed");

        assert_eq!(combos.len(), 3, "should return all 3 combos for the user");
        assert!(
            combos.iter().all(|c| c.user_id.num() == user_id.num()),
            "all returned combos should belong to the user"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn list_by_user_is_isolated_per_user(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user1 = setup_user(&state.pg_db_pool).await;
        let user2 = setup_user(&state.pg_db_pool).await;

        Combo::create(&state, user1, false, None, None, Some("User1 Combo 1"), None)
            .await
            .expect("create for user 1");
        Combo::create(&state, user1, false, None, None, Some("User1 Combo 2"), None)
            .await
            .expect("create for user 1");
        Combo::create(&state, user2, false, None, None, Some("User2 Combo 1"), None)
            .await
            .expect("create for user 2");

        let combos_user1 = Combo::list_by_user(&state, user1)
            .await
            .expect("list for user 1");
        let combos_user2 = Combo::list_by_user(&state, user2)
            .await
            .expect("list for user 2");

        assert_eq!(combos_user1.len(), 2, "user 1 should have exactly 2 combos");
        assert_eq!(combos_user2.len(), 1, "user 2 should have exactly 1 combo");
        Ok(())
    }

    #[sqlx::test]
    async fn list_by_user_empty_for_user_with_no_combos(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let combos = Combo::list_by_user(&state, user_id)
            .await
            .expect("list should succeed");

        assert!(combos.is_empty(), "user with no combos should return empty list");
        Ok(())
    }

    // ── Combo::update_combo ────────────────────────────────────────────────────

    #[sqlx::test]
    async fn update_combo_changes_owned_combo(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let created = Combo::create(
            &state,
            user_id,
            false,
            Some(1),
            Some("Old Category"),
            Some("Old Title"),
            Some("old-uuid"),
        )
        .await
        .expect("create should succeed");

        let user = User::lookup(&state, user_id)
            .await
            .expect("user should exist");

        let update_payload = ComboInfo {
            id: created.id,
            is_private: true,
            sort_order: Some(10),
            free_form_category: Some("New Category".to_string()),
            title: Some("New Title".to_string()),
            video_short_uuid: Some("new-uuid".to_string()),
        };

        let updated = Combo::update_combo(&state, &user, update_payload)
            .await
            .expect("update should succeed");

        assert!(updated.is_some(), "update should return Some for owned combo");
        let combo = updated.unwrap();
        assert_eq!(combo.id.num(), created.id.num());
        assert!(combo.is_private, "should update private flag");
        assert_eq!(combo.sort_order, Some(10), "should update sort_order");
        assert_eq!(combo.free_form_category, Some("New Category".to_string()));
        assert_eq!(combo.title, Some("New Title".to_string()));
        assert_eq!(combo.video_short_uuid, Some("new-uuid".to_string()));
        Ok(())
    }

    #[sqlx::test]
    async fn update_combo_clears_optional_fields(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let created = Combo::create(
            &state,
            user_id,
            false,
            Some(5),
            Some("Category"),
            Some("Title"),
            Some("uuid"),
        )
        .await
        .expect("create should succeed");

        let user = User::lookup(&state, user_id)
            .await
            .expect("user should exist");

        let update_payload = ComboInfo {
            id: created.id,
            is_private: false,
            sort_order: None,
            free_form_category: None,
            title: None,
            video_short_uuid: None,
        };

        let updated = Combo::update_combo(&state, &user, update_payload)
            .await
            .expect("update should succeed");

        assert!(updated.is_some());
        let combo = updated.unwrap();
        assert_eq!(combo.sort_order, None, "should clear sort_order");
        assert_eq!(combo.free_form_category, None, "should clear category");
        assert_eq!(combo.title, None, "should clear title");
        assert_eq!(combo.video_short_uuid, None, "should clear video uuid");
        Ok(())
    }

    #[sqlx::test]
    async fn update_combo_returns_none_for_non_owned_combo(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user1 = setup_user(&state.pg_db_pool).await;
        let user2 = setup_user(&state.pg_db_pool).await;

        let created = Combo::create(&state, user1, false, None, None, Some("Title"), None)
            .await
            .expect("create should succeed");

        let user2_record = User::lookup(&state, user2)
            .await
            .expect("user should exist");

        let update_payload = ComboInfo {
            id: created.id,
            is_private: true,
            sort_order: None,
            free_form_category: None,
            title: Some("Hacked Title".to_string()),
            video_short_uuid: None,
        };

        let result = Combo::update_combo(&state, &user2_record, update_payload)
            .await
            .expect("update should not error");

        assert!(
            result.is_none(),
            "update by non-owner should return None"
        );

        // Verify the combo was not actually updated
        let original = Combo::lookup(&state, created.id)
            .await
            .expect("lookup should succeed")
            .expect("combo should exist");
        assert_eq!(original.title, Some("Title".to_string()));
        Ok(())
    }

    #[sqlx::test]
    async fn update_combo_nonexistent_returns_none(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = setup_user(&state.pg_db_pool).await;

        let user = User::lookup(&state, user_id)
            .await
            .expect("user should exist");

        let update_payload = ComboInfo {
            id: ComboId(i64::MAX),
            is_private: false,
            sort_order: None,
            free_form_category: None,
            title: Some("Title".to_string()),
            video_short_uuid: None,
        };

        let result = Combo::update_combo(&state, &user, update_payload)
            .await
            .expect("update should not error");

        assert!(result.is_none(), "update of non-existent combo should return None");
        Ok(())
    }

    // ── Combo::from ────────────────────────────────────────────────────────────

    #[test]
    fn combo_from_row_maps_all_fields() {
        let row = ComboRow {
            id: 42,
            user_id: 10,
            is_private: true,
            sort_order: Some(3),
            free_form_category: Some("Footwork".to_string()),
            title: Some("Triple Step".to_string()),
            video_short_uuid: Some("vid-short-id".to_string()),
        };

        let combo = Combo::from(row);

        assert_eq!(combo.id.num(), 42);
        assert_eq!(combo.user_id.num(), 10);
        assert!(combo.is_private);
        assert_eq!(combo.sort_order, Some(3));
        assert_eq!(combo.free_form_category, Some("Footwork".to_string()));
        assert_eq!(combo.title, Some("Triple Step".to_string()));
        assert_eq!(combo.video_short_uuid, Some("vid-short-id".to_string()));
    }

    #[test]
    fn combo_id_num_roundtrips() {
        let id = ComboId(999);
        assert_eq!(id.num(), 999);
    }

    #[test]
    fn combo_id_for_test_creates_valid_id() {
        let id = ComboId::for_test(123);
        assert_eq!(id.num(), 123);
    }
}
