use crate::{
    api_endoints::club::{AddClubMemberRequest, AddClubVideoRequest},
    peertube::{
        channel::{PeerTubeChannelHandle, PeerTubeChannelId},
        playlist::PeerTubePlaylistId,
        user::PeerTubeAccountId,
    },
    user::UserId,
    AppState,
};
use sqlx::FromRow;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
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
    pub peertube_account_id: Option<i64>,
    pub is_admin: bool,
    pub public_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct PublicClubMemberInfo {
    pub user_id: UserId,
    pub public_name: String,
    pub membership: ClubMembership,
    pub peertube_account_id: Option<PeerTubeAccountId>,
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

    pub(crate) async fn lookup(state: &AppState, id: ClubId) -> Option<Club> {
        let maybe_club = sqlx::query_as!(
            ClubRow,
            r#"SELECT id, title, description, channel_id, main_playlist, channel_handle, web_link
            FROM clubs
            WHERE id = $1"#,
            id.num()
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
        club_id: ClubId,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PublicClubMemberInfo>, sqlx::Error> {
        let rows = sqlx::query_as!(
            UserJoinedClubRow,
            r#"
            SELECT uc.user_id, u.peertube_account_id, uc.is_admin, um.key_value AS public_name
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
        .await?;

        let members = rows
            .into_iter()
            .map(|record| PublicClubMemberInfo {
                user_id: record.user_id(),
                public_name: record.public_name,
                membership: ClubMembership::for_member(record.is_admin),
                peertube_account_id: record.peertube_account_id.map(PeerTubeAccountId),
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
