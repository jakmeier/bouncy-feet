use sqlx::prelude::FromRow;

use crate::{
    club::{ClubId, ClubRow},
    peertube::playlist::{PeerTubePlaylist, PeerTubePlaylistId},
    AppState,
};

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: PeerTubePlaylistId,
    #[allow(dead_code)]
    pub club_id: ClubId,
    pub is_private: bool,
    pub peertube_info: PlaylistInfo,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct PlaylistRow {
    pub club_id: i64,
    pub playlist_id: i64,
    pub playlist_short_uuid: String,
    pub is_private: bool,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct PlaylistInfo {
    pub id: PeerTubePlaylistId,
    pub short_uuid: String,
}

impl Playlist {
    pub(crate) async fn create(
        state: &AppState,
        club_id: ClubId,
        playlist_info: PeerTubePlaylist,
        is_private: bool,
    ) -> Result<Playlist, sqlx::Error> {
        let rec = sqlx::query_as!(
            PlaylistRow,
            r#"
            INSERT INTO club_playlists (club_id, playlist_id, playlist_short_uuid, is_private)
            VALUES ($1, $2, $3, $4)
            RETURNING club_id, playlist_id, playlist_short_uuid, is_private
            "#,
            club_id.num(),
            playlist_info.id.num(),
            playlist_info.short_uuid,
            is_private
        )
        .fetch_one(&state.pg_db_pool)
        .await?;

        Ok(Playlist::from(rec))
    }

    pub(crate) async fn lookup_club_playlist_by_peertube_id(
        state: &AppState,
        id: PeerTubePlaylistId,
    ) -> Option<Playlist> {
        let maybe_club = sqlx::query_as!(
            PlaylistRow,
            r#"SELECT club_id, playlist_id, playlist_short_uuid, is_private
            FROM club_playlists
            WHERE playlist_id = $1"#,
            id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed");

        maybe_club.map(Playlist::from)
    }

    pub(crate) async fn lookup_club_playlists(state: &AppState, club_id: ClubId) -> Vec<Playlist> {
        let clubs = sqlx::query_as!(
            PlaylistRow,
            r#"SELECT club_id, playlist_id, playlist_short_uuid, is_private
            FROM club_playlists
            WHERE club_id = $1"#,
            club_id.num()
        )
        .fetch_all(&state.pg_db_pool)
        .await
        .expect("DB query failed");

        clubs.into_iter().map(Playlist::from).collect()
    }

    pub(crate) async fn update_club_playlist_privacy(
        state: &AppState,
        playlist_id: PeerTubePlaylistId,
        is_private: bool,
    ) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!(
            r#"UPDATE club_playlists
            SET is_private = $1
            WHERE playlist_id = $2"#,
            is_private,
            playlist_id.num(),
        )
        .execute(&state.pg_db_pool)
        .await
    }

    pub(crate) fn playlist_info(self) -> PlaylistInfo {
        PlaylistInfo {
            id: self.peertube_info.id,
            short_uuid: self.peertube_info.short_uuid.clone(),
        }
    }
}

impl From<PlaylistRow> for Playlist {
    fn from(record: PlaylistRow) -> Self {
        Self {
            id: PeerTubePlaylistId(record.playlist_id),
            club_id: record.club_id(),
            is_private: record.is_private,
            peertube_info: PlaylistInfo {
                id: PeerTubePlaylistId(record.playlist_id),
                short_uuid: record.playlist_short_uuid,
            },
        }
    }
}

impl ClubRow {
    pub(crate) fn main_playlist_id(&self) -> Option<PeerTubePlaylistId> {
        self.main_playlist.map(PeerTubePlaylistId)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::{apply_migrations, make_test_state};
    use crate::peertube::playlist::PeerTubePlaylist;
    use crate::user::UserId;
    use sqlx::PgPool;

    /// Create a test club and return its ClubId
    async fn setup_club(pool: &PgPool, state: &AppState) -> ClubId {
        use crate::peertube::channel::{PeerTubeChannelHandle, PeerTubeChannelId};
        use crate::db::club::Club;

        let club = Club::create(
            state,
            "Test Club",
            "A test club",
            None,
            PeerTubeChannelId(999),
            PeerTubeChannelHandle("test-channel".to_string()),
            None,
        )
        .await
        .expect("failed to create test club");

        club.id
    }

    /// Create a test PeerTubePlaylist
    fn create_test_playlist(id: i64, short_uuid: &str) -> PeerTubePlaylist {
        PeerTubePlaylist {
            id: PeerTubePlaylistId(id),
            short_uuid: short_uuid.to_string(),
        }
    }

    // ── Playlist::create ────────────────────────────────────────────────────

    #[sqlx::test]
    async fn create_playlist_returns_valid_playlist(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let playlist_info = create_test_playlist(5001, "uuid-001");

        let playlist = Playlist::create(&state, club_id, playlist_info.clone(), false)
            .await
            .expect("create should succeed");

        assert_eq!(playlist.id.0, 5001);
        assert_eq!(playlist.club_id.num(), club_id.num());
        assert!(!playlist.is_private);
        assert_eq!(playlist.peertube_info.short_uuid, "uuid-001");
        Ok(())
    }

    #[sqlx::test]
    async fn create_playlist_can_be_private(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let playlist_info = create_test_playlist(5002, "uuid-002");

        let playlist = Playlist::create(&state, club_id, playlist_info, true)
            .await
            .expect("create should succeed");

        assert!(playlist.is_private, "playlist should be marked as private");
        Ok(())
    }

    #[sqlx::test]
    async fn create_multiple_playlists_for_same_club(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;

        let playlist1 = Playlist::create(
            &state,
            club_id,
            create_test_playlist(5003, "uuid-003"),
            false,
        )
        .await?;

        let playlist2 = Playlist::create(
            &state,
            club_id,
            create_test_playlist(5004, "uuid-004"),
            true,
        )
        .await?;

        assert_eq!(playlist1.club_id.num(), club_id.num());
        assert_eq!(playlist2.club_id.num(), club_id.num());
        assert_ne!(playlist1.id.0, playlist2.id.0, "playlists should have distinct IDs");
        Ok(())
    }

    #[sqlx::test]
    async fn create_playlist_preserves_uuid(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let uuid = "abc-123-def-456";
        let playlist_info = create_test_playlist(5005, uuid);

        let playlist = Playlist::create(&state, club_id, playlist_info, false)
            .await
            .expect("create should succeed");

        assert_eq!(
            playlist.peertube_info.short_uuid, uuid,
            "UUID should be preserved"
        );
        Ok(())
    }

    // ── Playlist::lookup_club_playlist_by_peertube_id ────────────────────────

    #[sqlx::test]
    async fn lookup_by_peertube_id_finds_created_playlist(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let peertube_id = 5006;
        let playlist_info = create_test_playlist(peertube_id, "uuid-006");

        Playlist::create(&state, club_id, playlist_info.clone(), false)
            .await
            .expect("create should succeed");

        let found = Playlist::lookup_club_playlist_by_peertube_id(&state, PeerTubePlaylistId(peertube_id))
            .await;

        assert!(found.is_some(), "should find the playlist");
        let found = found.unwrap();
        assert_eq!(found.id.0, peertube_id);
        assert_eq!(found.club_id.num(), club_id.num());
        Ok(())
    }

    #[sqlx::test]
    async fn lookup_by_peertube_id_returns_none_for_nonexistent(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);

        let found = Playlist::lookup_club_playlist_by_peertube_id(&state, PeerTubePlaylistId(99999))
            .await;

        assert!(found.is_none(), "should not find nonexistent playlist");
        Ok(())
    }

    #[sqlx::test]
    async fn lookup_by_peertube_id_returns_correct_privacy_status(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;

        let private_id = 5007;
        Playlist::create(
            &state,
            club_id,
            create_test_playlist(private_id, "uuid-007"),
            true,
        )
        .await?;

        let found = Playlist::lookup_club_playlist_by_peertube_id(&state, PeerTubePlaylistId(private_id))
            .await
            .unwrap();

        assert!(found.is_private, "should correctly preserve privacy status");
        Ok(())
    }

    #[sqlx::test]
    async fn lookup_by_peertube_id_isolates_between_clubs(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id1 = setup_club(&pool, &state).await;
        let club_id2 = setup_club(&pool, &state).await;

        let shared_peertube_id = 5008;
        Playlist::create(
            &state,
            club_id1,
            create_test_playlist(shared_peertube_id, "uuid-008"),
            false,
        )
        .await?;

        let found = Playlist::lookup_club_playlist_by_peertube_id(&state, PeerTubePlaylistId(shared_peertube_id))
            .await
            .unwrap();

        // Should find the playlist from club_id1
        assert_eq!(found.club_id.num(), club_id1.num());
        Ok(())
    }

    // ── Playlist::lookup_club_playlists ─────────────────────────────────────

    #[sqlx::test]
    async fn list_club_playlists_returns_created_playlists(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;

        Playlist::create(&state, club_id, create_test_playlist(5009, "uuid-009"), false).await?;
        Playlist::create(&state, club_id, create_test_playlist(5010, "uuid-010"), true).await?;

        let playlists = Playlist::lookup_club_playlists(&state, club_id).await;

        assert_eq!(playlists.len(), 2, "should return all playlists for club");
        Ok(())
    }

    #[sqlx::test]
    async fn list_club_playlists_returns_empty_for_club_with_no_playlists(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;

        let playlists = Playlist::lookup_club_playlists(&state, club_id).await;

        assert!(playlists.is_empty(), "new club should have no playlists");
        Ok(())
    }

    #[sqlx::test]
    async fn list_club_playlists_is_isolated_per_club(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id1 = setup_club(&pool, &state).await;
        let club_id2 = setup_club(&pool, &state).await;

        Playlist::create(&state, club_id1, create_test_playlist(5011, "uuid-011"), false).await?;
        Playlist::create(&state, club_id1, create_test_playlist(5012, "uuid-012"), false).await?;
        Playlist::create(&state, club_id2, create_test_playlist(5013, "uuid-013"), false).await?;

        let club1_playlists = Playlist::lookup_club_playlists(&state, club_id1).await;
        let club2_playlists = Playlist::lookup_club_playlists(&state, club_id2).await;

        assert_eq!(club1_playlists.len(), 2, "club1 should have 2 playlists");
        assert_eq!(club2_playlists.len(), 1, "club2 should have 1 playlist");

        for playlist in &club1_playlists {
            assert_eq!(playlist.club_id.num(), club_id1.num());
        }
        for playlist in &club2_playlists {
            assert_eq!(playlist.club_id.num(), club_id2.num());
        }
        Ok(())
    }

    #[sqlx::test]
    async fn list_club_playlists_includes_public_and_private(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;

        Playlist::create(&state, club_id, create_test_playlist(5014, "uuid-014"), false).await?;
        Playlist::create(&state, club_id, create_test_playlist(5015, "uuid-015"), true).await?;
        Playlist::create(&state, club_id, create_test_playlist(5016, "uuid-016"), false).await?;

        let playlists = Playlist::lookup_club_playlists(&state, club_id).await;

        let private_count = playlists.iter().filter(|p| p.is_private).count();
        let public_count = playlists.iter().filter(|p| !p.is_private).count();

        assert_eq!(private_count, 1, "should have 1 private playlist");
        assert_eq!(public_count, 2, "should have 2 public playlists");
        Ok(())
    }

    // ── Playlist::update_club_playlist_privacy ──────────────────────────────

    #[sqlx::test]
    async fn update_privacy_changes_is_private_flag(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let playlist_id = PeerTubePlaylistId(5017);

        Playlist::create(&state, club_id, create_test_playlist(5017, "uuid-017"), false).await?;

        // Change from public to private
        Playlist::update_club_playlist_privacy(&state, playlist_id, true).await?;

        let updated = Playlist::lookup_club_playlist_by_peertube_id(&state, playlist_id)
            .await
            .unwrap();

        assert!(updated.is_private, "playlist should now be private");
        Ok(())
    }

    #[sqlx::test]
    async fn update_privacy_from_private_to_public(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let playlist_id = PeerTubePlaylistId(5018);

        Playlist::create(&state, club_id, create_test_playlist(5018, "uuid-018"), true).await?;

        // Change from private to public
        Playlist::update_club_playlist_privacy(&state, playlist_id, false).await?;

        let updated = Playlist::lookup_club_playlist_by_peertube_id(&state, playlist_id)
            .await
            .unwrap();

        assert!(!updated.is_private, "playlist should now be public");
        Ok(())
    }

    #[sqlx::test]
    async fn update_privacy_for_nonexistent_playlist_succeeds_silently(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);

        // Updating a playlist that doesn't exist should not error
        let result = Playlist::update_club_playlist_privacy(&state, PeerTubePlaylistId(99999), true).await;

        assert!(result.is_ok(), "should succeed without error");
        Ok(())
    }

    #[sqlx::test]
    async fn update_privacy_preserves_other_fields(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let playlist_id = PeerTubePlaylistId(5019);
        let uuid = "uuid-019";

        Playlist::create(
            &state,
            club_id,
            create_test_playlist(5019, uuid),
            false,
        )
        .await?;

        Playlist::update_club_playlist_privacy(&state, playlist_id, true).await?;

        let updated = Playlist::lookup_club_playlist_by_peertube_id(&state, playlist_id)
            .await
            .unwrap();

        assert_eq!(updated.id.0, 5019, "id should be unchanged");
        assert_eq!(updated.club_id.num(), club_id.num(), "club_id should be unchanged");
        assert_eq!(updated.peertube_info.short_uuid, uuid, "uuid should be unchanged");
        assert!(updated.is_private, "privacy should be updated");
        Ok(())
    }

    // ── Playlist::playlist_info ─────────────────────────────────────────────

    #[sqlx::test]
    async fn playlist_info_extracts_correct_data(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let playlist_id = 5020;
        let uuid = "uuid-020";

        let playlist = Playlist::create(
            &state,
            club_id,
            create_test_playlist(playlist_id, uuid),
            false,
        )
        .await?;

        let info = playlist.playlist_info();

        assert_eq!(info.id.0, playlist_id);
        assert_eq!(info.short_uuid, uuid);
        Ok(())
    }

    // ── Playlist::From<PlaylistRow> ─────────────────────────────────────────

    #[sqlx::test]
    async fn from_playlist_row_converts_correctly(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;

        let created = Playlist::create(
            &state,
            club_id,
            create_test_playlist(5021, "uuid-021"),
            true,
        )
        .await?;

        // Verify the conversion happened correctly
        assert_eq!(created.id.0, 5021);
        assert_eq!(created.club_id.num(), club_id.num());
        assert!(created.is_private);
        assert_eq!(created.peertube_info.id.0, 5021);
        assert_eq!(created.peertube_info.short_uuid, "uuid-021");
        Ok(())
    }

    // ── Edge Cases ──────────────────────────────────────────────────────────

    #[sqlx::test]
    async fn playlist_with_special_characters_in_uuid(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let special_uuid = "abc-123_xyz.456";

        let playlist = Playlist::create(
            &state,
            club_id,
            create_test_playlist(5022, special_uuid),
            false,
        )
        .await?;

        assert_eq!(playlist.peertube_info.short_uuid, special_uuid);
        Ok(())
    }

    #[sqlx::test]
    async fn large_playlist_id_values(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id = setup_club(&pool, &state).await;
        let large_id = i64::MAX - 1000;

        let playlist = Playlist::create(
            &state,
            club_id,
            create_test_playlist(large_id, "uuid-large"),
            false,
        )
        .await?;

        assert_eq!(playlist.id.0, large_id);

        let found = Playlist::lookup_club_playlist_by_peertube_id(&state, PeerTubePlaylistId(large_id))
            .await;

        assert!(found.is_some(), "should be able to lookup large id");
        Ok(())
    }

    #[sqlx::test]
    async fn access_control_through_club_isolation(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool.clone());
        let club_id1 = setup_club(&pool, &state).await;
        let club_id2 = setup_club(&pool, &state).await;

        let playlist1 = Playlist::create(
            &state,
            club_id1,
            create_test_playlist(5023, "uuid-023"),
            false,
        )
        .await?;

        let playlist2 = Playlist::create(
            &state,
            club_id2,
            create_test_playlist(5024, "uuid-024"),
            true,
        )
        .await?;

        // Club1's playlists should only include playlist1
        let club1_playlists = Playlist::lookup_club_playlists(&state, club_id1).await;
        assert_eq!(club1_playlists.len(), 1);
        assert_eq!(club1_playlists[0].id.0, 5023);

        // Club2's playlists should only include playlist2
        let club2_playlists = Playlist::lookup_club_playlists(&state, club_id2).await;
        assert_eq!(club2_playlists.len(), 1);
        assert_eq!(club2_playlists[0].id.0, 5024);

        Ok(())
    }
}
