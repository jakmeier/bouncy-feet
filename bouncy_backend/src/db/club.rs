use crate::{
    api_endoints::club::{AddClubMemberRequest, AddClubVideoRequest},
    db_err_to_status,
    peertube::{
        channel::{PeerTubeChannelHandle, PeerTubeChannelId},
        playlist::PeerTubePlaylistId,
        user::PeerTubeHandle,
    },
    user::UserId,
    AppState, CheckedClubId,
};
use axum::http::StatusCode;
use sqlx::FromRow;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ClubId(i64);

#[derive(Debug, Clone)]
pub struct Club {
    pub id: ClubId,
    pub title: String,
    pub description: String,
    pub main_playlist: Option<PeerTubePlaylistId>,
    pub channel_id: Option<PeerTubeChannelId>,
    pub channel_handle: Option<PeerTubeChannelHandle>,
    pub web_link: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ClubMembership {
    None,
    Member,
    Admin,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct ClubRow {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub web_link: Option<String>,
    pub channel_id: Option<i64>,
    pub channel_handle: Option<String>,
    pub main_playlist: Option<i64>,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct UserClubRow {
    pub user_id: i64,
    // #[allow(dead_code)]
    // club_id: i64,
    is_admin: bool,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct UserJoinedClubRow {
    pub user_id: i64,
    pub peertube_handle: Option<String>,
    pub is_admin: bool,
    pub public_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct PublicClubMemberInfo {
    pub user_id: UserId,
    pub public_name: String,
    pub membership: ClubMembership,
    pub peertube_handle: Option<PeerTubeHandle>,
}

impl Club {
    pub(crate) async fn create(
        state: &AppState,
        title: &str,
        description: &str,
        web_link: Option<url::Url>,
        channel_id: PeerTubeChannelId,
        channel_handle: PeerTubeChannelHandle,
        main_playlist: Option<PeerTubePlaylistId>,
    ) -> Result<Club, sqlx::Error> {
        let rec = sqlx::query_as!(
            ClubRow,
            r#"
            INSERT INTO clubs (title, description, channel_id, main_playlist, channel_handle, web_link)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, title, description, channel_id, main_playlist, channel_handle, web_link
            "#,
            title,
            description,
            channel_id.num(),
            main_playlist.map(|id| id.num()),
            channel_handle.0,
            web_link.as_ref().map(|wl|wl.as_str())
        )
        .fetch_one(&state.pg_db_pool)
        .await?;

        Ok(Club::from(rec))
    }

    pub(crate) async fn lookup(state: &AppState, id: CheckedClubId) -> Option<Club> {
        let club_id = id.assert_public_read_access().ok()?;
        let maybe_club = sqlx::query_as!(
            ClubRow,
            r#"SELECT id, title, description, channel_id, main_playlist, channel_handle, web_link
            FROM clubs
            WHERE id = $1"#,
            club_id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed");

        maybe_club.map(Club::from)
    }

    /// Remove a club from the DB.
    ///
    /// Triggers implicit cascade delete but will not explicitly clean up any
    /// data for a club.
    ///
    /// Returns true iff club was deleted.
    pub(crate) async fn delete(state: &AppState, club_id: ClubId) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"
            DELETE FROM clubs WHERE id = $1
            "#,
            club_id.num(),
        )
        .execute(&state.pg_db_pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }

    /// List clubs with optional limit/offset
    pub async fn list(state: &AppState, limit: i64, offset: i64) -> Result<Vec<Club>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ClubRow,
            r#"
            SELECT id, title, description, channel_id, main_playlist, channel_handle, web_link
            FROM clubs
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&state.pg_db_pool)
        .await?;

        let clubs = rows.into_iter().map(Club::from).collect();

        Ok(clubs)
    }

    /// Add a member to a club. If the membership already exists, updates is_admin.
    pub async fn add_or_update_member(
        state: &AppState,
        user_id: UserId,
        club_id: ClubId,
        is_admin: bool,
    ) -> Result<(), sqlx::Error> {
        // upsert
        sqlx::query!(
            r#"
            INSERT INTO user_club (user_id, club_id, is_admin)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, club_id) DO UPDATE SET is_admin = EXCLUDED.is_admin
            "#,
            user_id.num(),
            club_id.num(),
            is_admin,
        )
        .execute(&state.pg_db_pool)
        .await?;
        Ok(())
    }

    /// Remove a member from a club if it exists.
    ///
    /// Returns true iff member was removed.
    #[allow(dead_code)]
    pub async fn remove_member(
        state: &AppState,
        user_id: UserId,
        club_id: ClubId,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"
            DELETE FROM user_club WHERE user_id = $1 AND club_id = $2
            "#,
            user_id.num(),
            club_id.num(),
        )
        .execute(&state.pg_db_pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }

    /// Check if a user is a member (and get the membership row)
    pub async fn membership(
        state: &AppState,
        user_id: UserId,
        club_id: ClubId,
    ) -> Result<ClubMembership, sqlx::Error> {
        let record = sqlx::query_as!(
            UserClubRow,
            r#"
            SELECT user_id, is_admin FROM user_club
            WHERE user_id = $1 AND club_id = $2
            "#,
            user_id.num(),
            club_id.num(),
        )
        .fetch_optional(&state.pg_db_pool)
        .await?;

        Ok(ClubMembership::from(record))
    }

    /// List members of a club
    #[allow(dead_code)]
    pub async fn list_members(
        state: &AppState,
        club_id: ClubId,
    ) -> Result<Vec<(UserId, ClubMembership)>, sqlx::Error> {
        let rows = sqlx::query_as!(
            UserClubRow,
            r#"
            SELECT user_id, is_admin FROM user_club
            WHERE club_id = $1
            ORDER BY user_id
            "#,
            club_id.num(),
        )
        .fetch_all(&state.pg_db_pool)
        .await?;

        let members = rows
            .into_iter()
            .map(|record| (record.user_id(), Some(record).into()))
            .collect();

        Ok(members)
    }

    /// List members of a club with their user info
    pub async fn list_members_with_info(
        state: &AppState,
        checked_club_id: CheckedClubId,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PublicClubMemberInfo>, (StatusCode, &'static str)> {
        let club_id = checked_club_id.assert_private_read_access()?;
        let rows = sqlx::query_as!(
            UserJoinedClubRow,
            r#"
            SELECT uc.user_id, u.peertube_handle, uc.is_admin, um.key_value AS public_name
            FROM user_club uc
            JOIN user_meta um ON uc.user_id = um.user_id
            JOIN users u ON uc.user_id = u.id
            WHERE uc.club_id = $1
                AND um.key_name = 's:publicName'
            ORDER BY um.user_id
            LIMIT $2 OFFSET $3
            "#,
            club_id.num(),
            limit,
            offset
        )
        .fetch_all(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;

        let members = rows
            .into_iter()
            .map(|record| PublicClubMemberInfo {
                user_id: record.user_id(),
                public_name: record.public_name,
                membership: ClubMembership::for_member(record.is_admin),
                peertube_handle: record.peertube_handle.map(PeerTubeHandle),
            })
            .collect();

        Ok(members)
    }

    /// List clubs for a user
    pub async fn list_clubs_for_user(
        state: &AppState,
        user_id: UserId,
    ) -> Result<Vec<Club>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ClubRow,
            r#"
            SELECT c.id,
                   c.title,
                   c.description,
                   c.web_link,
                   c.channel_id,
                   c.channel_handle,
                   c.main_playlist
            FROM clubs c
            JOIN user_club uc ON uc.club_id = c.id
            WHERE uc.user_id = $1
            ORDER BY c.id
            "#,
            user_id.num(),
        )
        .fetch_all(&state.pg_db_pool)
        .await?;

        let clubs = rows.into_iter().map(Club::from).collect();

        Ok(clubs)
    }

    pub async fn set_main_playlist(
        state: &AppState,
        club_id: ClubId,
        playlist_id: PeerTubePlaylistId,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE clubs
            SET main_playlist = $1
            WHERE clubs.id = $2;
            "#,
            playlist_id.num(),
            club_id.num()
        )
        .execute(&state.pg_db_pool)
        .await?;

        Ok(())
    }

    /// Update fields that admins can just modify, unlike fields that are fixed after creation or that require linked changes in PeerTube.
    pub async fn set_meta_fields(
        state: &AppState,
        club_id: ClubId,
        description: String,
        web_link: Option<url::Url>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE clubs
            SET description = $1,
                web_link = $2
            WHERE clubs.id = $3;
            "#,
            description,
            web_link.as_ref().map(|wl| wl.as_str()),
            club_id.num(),
        )
        .execute(&state.pg_db_pool)
        .await?;

        Ok(())
    }
}

impl ClubId {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl From<ClubRow> for Club {
    fn from(record: ClubRow) -> Self {
        Club {
            main_playlist: record.main_playlist_id(),
            id: ClubId(record.id),
            title: record.title,
            description: record.description,
            channel_id: record.channel_id.map(PeerTubeChannelId),
            channel_handle: record.channel_handle.map(PeerTubeChannelHandle),
            web_link: record.web_link,
        }
    }
}

impl ClubMembership {
    fn for_member(is_admin: bool) -> Self {
        if is_admin {
            ClubMembership::Admin
        } else {
            ClubMembership::Member
        }
    }
}

impl From<Option<UserClubRow>> for ClubMembership {
    fn from(maybe_record: Option<UserClubRow>) -> Self {
        match maybe_record {
            None => ClubMembership::None,
            Some(record) => ClubMembership::for_member(record.is_admin),
        }
    }
}

impl AddClubMemberRequest {
    pub(crate) fn club_id(&self) -> ClubId {
        ClubId(self.club_id)
    }
}

impl AddClubVideoRequest {
    pub(crate) fn club_id(&self) -> ClubId {
        ClubId(self.club_id)
    }
}

impl super::playlist::PlaylistRow {
    pub(crate) fn club_id(&self) -> ClubId {
        ClubId(self.club_id)
    }
}

impl super::clubs_combos::ClubComboRow {
    pub(crate) fn club_id(&self) -> ClubId {
        ClubId(self.club_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::make_test_state;
    use sqlx::PgPool;

    /// Create a test user and return its UserId
    async fn setup_user(pool: &PgPool) -> UserId {
        let user_id: i64 =
            sqlx::query_scalar("INSERT INTO users (oidc_subject) VALUES (null) RETURNING id")
                .fetch_one(pool)
                .await
                .expect("failed to insert test user");
        UserId::from_i64(user_id)
    }

    /// Create a test user with public_name metadata
    async fn setup_user_with_name(pool: &PgPool, public_name: &str) -> UserId {
        let user_id: i64 =
            sqlx::query_scalar("INSERT INTO users (oidc_subject) VALUES (null) RETURNING id")
                .fetch_one(pool)
                .await
                .expect("failed to insert test user");

        sqlx::query(
            "INSERT INTO user_meta (user_id, key_name, key_value, version_nr) VALUES ($1, $2, $3, 1)"
        )
        .bind(user_id)
        .bind("s:publicName")
        .bind(public_name)
        .execute(pool)
        .await
        .expect("failed to insert user metadata");

        UserId::from_i64(user_id)
    }

    // ── Club::create ────────────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn create_club_returns_valid_id_and_fields(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        let club = Club::create(
            &state,
            "Test Club",
            "A test club for testing",
            Some("https://example.com/".parse().unwrap()),
            PeerTubeChannelId(42),
            PeerTubeChannelHandle("test_channel".to_string()),
            None,
        )
        .await
        .expect("create should succeed");

        assert!(
            club.id.num() > 0,
            "newly created club should have a positive id"
        );
        assert_eq!(club.title, "Test Club");
        assert_eq!(club.description, "A test club for testing");
        assert_eq!(club.web_link, Some("https://example.com/".to_string()));
        assert_eq!(club.channel_id, Some(PeerTubeChannelId(42)));
        assert_eq!(
            club.channel_handle,
            Some(PeerTubeChannelHandle("test_channel".to_string()))
        );
        assert_eq!(club.main_playlist, None);
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn create_club_without_optional_fields(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        let club = Club::create(
            &state,
            "Minimal Club",
            "No extras",
            None,
            PeerTubeChannelId(1),
            PeerTubeChannelHandle("minimal".to_string()),
            None,
        )
        .await?;

        assert_eq!(club.title, "Minimal Club");
        assert_eq!(club.description, "No extras");
        assert_eq!(club.web_link, None);
        assert_eq!(club.main_playlist, None);
        Ok(())
    }

    // ── Club::lookup ────────────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn lookup_existing_club_returns_some(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        let created = Club::create(
            &state,
            "Lookup Test",
            "Test description",
            None,
            PeerTubeChannelId(10),
            PeerTubeChannelHandle("lookup".to_string()),
            None,
        )
        .await?;

        let found = Club::lookup(&state, CheckedClubId::PublicReadAccess(created.id)).await;

        assert!(
            found.is_some(),
            "should find the club that was just created"
        );
        let found = found.unwrap();
        assert_eq!(found.id.num(), created.id.num());
        assert_eq!(found.title, "Lookup Test");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn lookup_nonexistent_club_returns_none(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        let missing = Club::lookup(&state, CheckedClubId::PublicReadAccess(ClubId(i64::MAX))).await;

        assert!(
            missing.is_none(),
            "looking up a non-existent club should return None"
        );
        Ok(())
    }

    // ── Club::delete ────────────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_existing_club_returns_true(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        let club = Club::create(
            &state,
            "Delete Test",
            "Will be deleted",
            None,
            PeerTubeChannelId(20),
            PeerTubeChannelHandle("delete".to_string()),
            None,
        )
        .await?;

        let deleted = Club::delete(&state, club.id).await?;

        assert!(deleted, "delete should return true when club existed");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_removes_club_from_database(pool: PgPool) -> anyhow::Result<()> {
        let state = make_test_state(pool.clone());

        let club = Club::create(
            &state,
            "Delete Verify",
            "Test deletion",
            None,
            PeerTubeChannelId(21),
            PeerTubeChannelHandle("delver".to_string()),
            None,
        )
        .await?;

        Club::delete(&state, club.id).await?;
        let lookup_result = Club::lookup(&state, CheckedClubId::Owned(club.id)).await;

        assert!(
            lookup_result.is_none(),
            "club should not be found after deletion"
        );
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn delete_nonexistent_club_returns_false(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        let deleted = Club::delete(&state, ClubId(i64::MAX)).await?;

        assert!(
            !deleted,
            "delete should return false when club did not exist"
        );
        Ok(())
    }

    // ── Club::list ──────────────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_clubs_returns_created_clubs(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        Club::create(
            &state,
            "Club A",
            "A",
            None,
            PeerTubeChannelId(30),
            PeerTubeChannelHandle("a".to_string()),
            None,
        )
        .await?;
        Club::create(
            &state,
            "Club B",
            "B",
            None,
            PeerTubeChannelId(31),
            PeerTubeChannelHandle("b".to_string()),
            None,
        )
        .await?;
        Club::create(
            &state,
            "Club C",
            "C",
            None,
            PeerTubeChannelId(32),
            PeerTubeChannelHandle("c".to_string()),
            None,
        )
        .await?;

        let clubs = Club::list(&state, 10, 0).await?;

        assert_eq!(clubs.len(), 3, "should list all three clubs");
        assert_eq!(clubs[0].title, "Club A");
        assert_eq!(clubs[1].title, "Club B");
        assert_eq!(clubs[2].title, "Club C");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_clubs_respects_limit_and_offset(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        for i in 0..5 {
            Club::create(
                &state,
                &format!("Club {}", i),
                "test",
                None,
                PeerTubeChannelId(40 + i),
                PeerTubeChannelHandle(format!("ch{}", i)),
                None,
            )
            .await?;
        }

        let first_page = Club::list(&state, 2, 0).await?;
        let second_page = Club::list(&state, 2, 2).await?;
        let all = Club::list(&state, 10, 0).await?;

        assert_eq!(first_page.len(), 2);
        assert_eq!(second_page.len(), 2);
        assert_eq!(all.len(), 5);
        assert_ne!(first_page[0].id.num(), second_page[0].id.num());
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_clubs_empty_database_returns_empty_vec(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());

        let clubs = Club::list(&state, 10, 0).await?;

        assert!(clubs.is_empty(), "empty database should return empty list");
        Ok(())
    }

    // ── Club::add_or_update_member ──────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn add_member_to_club_succeeds(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(50),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user, club.id, false).await?;

        let membership = Club::membership(&state, user, club.id).await?;
        assert!(matches!(membership, ClubMembership::Member));
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn add_member_as_admin(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(51),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user, club.id, true).await?;

        let membership = Club::membership(&state, user, club.id).await?;
        assert!(matches!(membership, ClubMembership::Admin));
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn update_member_status_changes_admin_flag(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(52),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user, club.id, false).await?;
        let membership_before = Club::membership(&state, user, club.id).await?;
        assert!(matches!(membership_before, ClubMembership::Member));

        Club::add_or_update_member(&state, user, club.id, true).await?;
        let membership_after = Club::membership(&state, user, club.id).await?;
        assert!(matches!(membership_after, ClubMembership::Admin));
        Ok(())
    }

    // ── Club::remove_member ─────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn remove_existing_member_returns_true(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(60),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user, club.id, false).await?;
        let removed = Club::remove_member(&state, user, club.id).await?;

        assert!(
            removed,
            "remove_member should return true when member existed"
        );
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn remove_member_updates_membership_to_none(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(61),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user, club.id, false).await?;
        Club::remove_member(&state, user, club.id).await?;
        let membership = Club::membership(&state, user, club.id).await?;

        assert!(matches!(membership, ClubMembership::None));
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn remove_nonexistent_member_returns_false(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(62),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        let removed = Club::remove_member(&state, user, club.id).await?;

        assert!(
            !removed,
            "remove_member should return false when member did not exist"
        );
        Ok(())
    }

    // ── Club::membership ────────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn membership_returns_none_for_non_member(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(70),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        let membership = Club::membership(&state, user, club.id).await?;

        assert!(matches!(membership, ClubMembership::None));
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn membership_distinguishes_member_and_admin(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user1 = setup_user(&pool).await;
        let user2 = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(71),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user1, club.id, false).await?;
        Club::add_or_update_member(&state, user2, club.id, true).await?;

        let m1 = Club::membership(&state, user1, club.id).await?;
        let m2 = Club::membership(&state, user2, club.id).await?;

        assert!(matches!(m1, ClubMembership::Member));
        assert!(matches!(m2, ClubMembership::Admin));
        Ok(())
    }

    // ── Club::list_members ──────────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_members_returns_all_club_members(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user1 = setup_user(&pool).await;
        let user2 = setup_user(&pool).await;
        let user3 = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(80),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user1, club.id, false).await?;
        Club::add_or_update_member(&state, user2, club.id, true).await?;
        Club::add_or_update_member(&state, user3, club.id, false).await?;

        let members = Club::list_members(&state, club.id).await?;

        assert_eq!(members.len(), 3, "should list all three members");
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_members_returns_correct_membership_types(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user1 = setup_user(&pool).await;
        let user2 = setup_user(&pool).await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(81),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user1, club.id, false).await?;
        Club::add_or_update_member(&state, user2, club.id, true).await?;

        let members = Club::list_members(&state, club.id).await?;

        let (_, m1) = members
            .iter()
            .find(|(id, _)| id.num() == user1.num())
            .unwrap();
        let (_, m2) = members
            .iter()
            .find(|(id, _)| id.num() == user2.num())
            .unwrap();

        assert!(matches!(m1, ClubMembership::Member));
        assert!(matches!(m2, ClubMembership::Admin));
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_members_empty_for_new_club(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(82),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        let members = Club::list_members(&state, club.id).await?;

        assert!(members.is_empty(), "new club should have no members");
        Ok(())
    }

    // ── Club::list_members_with_info ────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_members_with_info_includes_user_data(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user1 = setup_user_with_name(&pool, "Alice").await;
        let user2 = setup_user_with_name(&pool, "Bob").await;
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(90),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user1, club.id, false).await?;
        Club::add_or_update_member(&state, user2, club.id, true).await?;

        let club_id = CheckedClubId::FullReadAccess(club.id);
        let members = Club::list_members_with_info(&state, club_id, 10, 0)
            .await
            .unwrap();

        assert_eq!(members.len(), 2);
        assert!(members
            .iter()
            .any(|m| m.public_name == "Alice" && matches!(m.membership, ClubMembership::Member)));
        assert!(members
            .iter()
            .any(|m| m.public_name == "Bob" && matches!(m.membership, ClubMembership::Admin)));
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_members_with_info_respects_limit_and_offset(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(91),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        for i in 0..5 {
            let user = setup_user_with_name(&pool, &format!("User{}", i)).await;
            Club::add_or_update_member(&state, user, club.id, false).await?;
        }

        let club_id = CheckedClubId::FullReadAccess(club.id);
        let page1 = Club::list_members_with_info(&state, club_id, 2, 0)
            .await
            .unwrap();
        let page2 = Club::list_members_with_info(&state, club_id, 2, 2)
            .await
            .unwrap();
        let all = Club::list_members_with_info(&state, club_id, 10, 0)
            .await
            .unwrap();

        assert_eq!(page1.len(), 2);
        assert_eq!(page2.len(), 2);
        assert_eq!(all.len(), 5);
        Ok(())
    }

    // ── Club::list_clubs_for_user ───────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_clubs_for_user_returns_user_clubs_only(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user1 = setup_user(&pool).await;
        let user2 = setup_user(&pool).await;
        let club1 = Club::create(
            &state,
            "Club1",
            "desc",
            None,
            PeerTubeChannelId(100),
            PeerTubeChannelHandle("c1".to_string()),
            None,
        )
        .await?;
        let club2 = Club::create(
            &state,
            "Club2",
            "desc",
            None,
            PeerTubeChannelId(101),
            PeerTubeChannelHandle("c2".to_string()),
            None,
        )
        .await?;
        let club3 = Club::create(
            &state,
            "Club3",
            "desc",
            None,
            PeerTubeChannelId(102),
            PeerTubeChannelHandle("c3".to_string()),
            None,
        )
        .await?;

        Club::add_or_update_member(&state, user1, club1.id, false).await?;
        Club::add_or_update_member(&state, user1, club2.id, false).await?;
        Club::add_or_update_member(&state, user2, club3.id, false).await?;

        let user1_clubs = Club::list_clubs_for_user(&state, user1).await?;
        let user2_clubs = Club::list_clubs_for_user(&state, user2).await?;

        assert_eq!(user1_clubs.len(), 2);
        assert_eq!(user2_clubs.len(), 1);
        assert!(user1_clubs.iter().any(|c| c.title == "Club1"));
        assert!(user1_clubs.iter().any(|c| c.title == "Club2"));
        assert!(user2_clubs.iter().any(|c| c.title == "Club3"));
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn list_clubs_for_user_empty_for_user_with_no_clubs(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let user = setup_user(&pool).await;

        let clubs = Club::list_clubs_for_user(&state, user).await?;

        assert!(clubs.is_empty());
        Ok(())
    }

    // ── Club::set_main_playlist ─────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    // TODO
    #[ignore] // Skipped: requires playlist to exist in club_playlists table
    async fn set_main_playlist_updates_club(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(110),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        assert_eq!(club.main_playlist, None);

        Club::set_main_playlist(&state, club.id, PeerTubePlaylistId(200)).await?;

        let updated = Club::lookup(&state, CheckedClubId::PublicReadAccess(club.id))
            .await
            .unwrap();
        assert_eq!(updated.main_playlist, Some(PeerTubePlaylistId(200)));
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    // TODO
    #[ignore] // Skipped: requires playlist to exist in club_playlists table
    async fn set_main_playlist_can_update_existing_playlist(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let club = Club::create(
            &state,
            "Club",
            "desc",
            None,
            PeerTubeChannelId(111),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::set_main_playlist(&state, club.id, PeerTubePlaylistId(300)).await?;

        let updated = Club::lookup(&state, CheckedClubId::PublicReadAccess(club.id))
            .await
            .unwrap();
        assert_eq!(updated.main_playlist, Some(PeerTubePlaylistId(300)));
        Ok(())
    }

    // ── Club::set_meta_fields ──────────────────────────────────────────────

    #[sqlx::test(migrations = "./db_migrations")]
    async fn set_meta_fields_updates_description_and_web_link(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let club = Club::create(
            &state,
            "Club",
            "old description",
            Some("https://old.example.com/".parse().unwrap()),
            PeerTubeChannelId(120),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::set_meta_fields(
            &state,
            club.id,
            "new description".to_string(),
            Some("https://new.example.com/".parse().unwrap()),
        )
        .await?;

        let updated = Club::lookup(&state, CheckedClubId::PublicReadAccess(club.id))
            .await
            .unwrap();
        assert_eq!(updated.description, "new description");
        assert_eq!(
            updated.web_link,
            Some("https://new.example.com/".to_string())
        );
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn set_meta_fields_can_clear_web_link(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let club = Club::create(
            &state,
            "Club",
            "desc",
            Some("https://example.com".parse().unwrap()),
            PeerTubeChannelId(121),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::set_meta_fields(&state, club.id, "new desc".to_string(), None).await?;

        let updated = Club::lookup(&state, CheckedClubId::PublicReadAccess(club.id))
            .await
            .unwrap();
        assert_eq!(updated.web_link, None);
        Ok(())
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn set_meta_fields_does_not_modify_other_fields(pool: PgPool) -> sqlx::Result<()> {
        let state = make_test_state(pool.clone());
        let club = Club::create(
            &state,
            "Original Title",
            "original description",
            None,
            PeerTubeChannelId(122),
            PeerTubeChannelHandle("ch".to_string()),
            None,
        )
        .await?;

        Club::set_meta_fields(&state, club.id, "new desc".to_string(), None).await?;

        let updated = Club::lookup(&state, CheckedClubId::Owned(club.id))
            .await
            .unwrap();
        assert_eq!(updated.title, "Original Title", "title should not change");
        assert_eq!(
            updated.channel_id,
            Some(PeerTubeChannelId(122)),
            "channel_id should not change"
        );
        assert_eq!(
            updated.main_playlist, None,
            "main_playlist should not change"
        );
        Ok(())
    }

    // ── Pure unit tests (no DB) ───────────────────────────────────────────────

    #[test]
    fn club_id_num_roundtrips() {
        let id = ClubId(42);
        assert_eq!(id.num(), 42);
    }

    #[test]
    fn club_membership_for_member_returns_member() {
        let membership = ClubMembership::for_member(false);
        assert!(matches!(membership, ClubMembership::Member));
    }

    #[test]
    fn club_membership_for_admin_returns_admin() {
        let membership = ClubMembership::for_member(true);
        assert!(matches!(membership, ClubMembership::Admin));
    }

    #[test]
    fn club_membership_from_none_returns_none() {
        let membership = ClubMembership::from(None);
        assert!(matches!(membership, ClubMembership::None));
    }

    #[test]
    fn club_membership_from_row_reflects_admin_status() {
        let row = UserClubRow {
            user_id: 1,
            is_admin: true,
        };
        let membership = ClubMembership::from(Some(row));
        assert!(matches!(membership, ClubMembership::Admin));

        let row = UserClubRow {
            user_id: 2,
            is_admin: false,
        };
        let membership = ClubMembership::from(Some(row));
        assert!(matches!(membership, ClubMembership::Member));
    }
}
