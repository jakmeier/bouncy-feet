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
