use sqlx::prelude::FromRow;

use crate::{
    club::{ClubId, ClubRow},
    peertube::playlist::{PeerTubePlaylist, PeerTubePlaylistId},
    AppState,
};

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(transparent)]
pub struct PlaylistId(i64);

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: PlaylistId,
    #[allow(dead_code)]
    pub club_id: ClubId,
    pub is_private: bool,
    pub peertube_info: PlaylistInfo,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct PlaylistRow {
    pub id: i64,
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
            RETURNING id, club_id, playlist_id, playlist_short_uuid, is_private
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

    pub(crate) async fn lookup_club_playlist(state: &AppState, id: PlaylistId) -> Option<Playlist> {
        let maybe_club = sqlx::query_as!(
            PlaylistRow,
            r#"SELECT id, club_id, playlist_id, playlist_short_uuid, is_private
            FROM club_playlists
            WHERE id = $1"#,
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
            r#"SELECT id, club_id, playlist_id, playlist_short_uuid, is_private
            FROM club_playlists
            WHERE club_id = $1"#,
            club_id.num()
        )
        .fetch_all(&state.pg_db_pool)
        .await
        .expect("DB query failed");

        clubs.into_iter().map(Playlist::from).collect()
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
            id: PlaylistId(record.id),
            club_id: record.club_id(),
            is_private: record.is_private,
            peertube_info: PlaylistInfo {
                id: PeerTubePlaylistId(record.playlist_id),
                short_uuid: record.playlist_short_uuid,
            },
        }
    }
}

impl PlaylistId {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl ClubRow {
    pub(crate) fn main_playlist_id(&self) -> Option<PlaylistId> {
        self.main_playlist.map(PlaylistId)
    }
}
