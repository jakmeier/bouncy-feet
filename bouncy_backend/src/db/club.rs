use crate::{user::UserId, AppState};
use sqlx::FromRow;

#[derive(Clone, Copy, Debug)]
pub struct ClubId(i64);

#[derive(Debug, Clone)]
pub struct Club {
    pub id: ClubId,
    pub title: String,
    pub description: String,
    pub public_playlist: String,
    pub private_playlist: String,
}

#[derive(Debug, Clone)]
pub enum ClubMembership {
    None,
    Member,
    Admin,
}

#[derive(Debug, Clone, FromRow)]
struct ClubRow {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub public_playlist: String,
    pub private_playlist: String,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct UserClubRow {
    pub user_id: i64,
    #[allow(dead_code)]
    club_id: i64,
    is_admin: bool,
}

impl Club {
    pub(crate) async fn create(
        state: &AppState,
        title: &str,
        description: &str,
        public_playlist: &str,
        private_playlist: &str,
    ) -> Result<Club, sqlx::Error> {
        let rec = sqlx::query_as!(
            ClubRow,
            r#"
            INSERT INTO clubs (title, description, public_playlist, private_playlist)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, description, public_playlist, private_playlist
            "#,
            title,
            description,
            public_playlist,
            private_playlist
        )
        .fetch_one(&state.pg_db_pool)
        .await?;

        Ok(Club::from(rec))
    }

    #[allow(dead_code)]
    pub(crate) async fn lookup(state: &AppState, id: ClubId) -> Option<Club> {
        let maybe_club = sqlx::query_as!(
            ClubRow,
            r#"SELECT id, title, description, public_playlist, private_playlist
            FROM clubs
            WHERE id = $1"#,
            id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed");

        maybe_club.map(Club::from)
    }

    /// List clubs with optional limit/offset
    pub async fn list(state: &AppState, limit: i64, offset: i64) -> Result<Vec<Club>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ClubRow,
            r#"
            SELECT id, title, description, public_playlist, private_playlist
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
    #[allow(dead_code)]
    pub async fn membership(
        state: &AppState,
        user_id: UserId,
        club_id: ClubId,
    ) -> Result<ClubMembership, sqlx::Error> {
        let record = sqlx::query_as!(
            UserClubRow,
            r#"
            SELECT user_id, club_id, is_admin FROM user_club
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
            SELECT user_id, club_id, is_admin FROM user_club
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

    /// List clubs for a user
    pub async fn list_clubs_for_user(
        state: &AppState,
        user_id: UserId,
    ) -> Result<Vec<Club>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ClubRow,
            r#"
            SELECT c.id, c.title, c.description, c.public_playlist, c.private_playlist
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
}

impl ClubId {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl From<ClubRow> for Club {
    fn from(record: ClubRow) -> Self {
        Club {
            id: ClubId(record.id),
            title: record.title,
            description: record.description,
            public_playlist: record.public_playlist,
            private_playlist: record.private_playlist,
        }
    }
}

impl From<Option<UserClubRow>> for ClubMembership {
    fn from(maybe_record: Option<UserClubRow>) -> Self {
        match maybe_record {
            None => ClubMembership::None,
            Some(record) if record.is_admin => ClubMembership::Admin,
            Some(_) => ClubMembership::Member,
        }
    }
}
