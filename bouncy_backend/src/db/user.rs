use axum_oidc::OidcClaims;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api_endoints::club::AddClubMemberRequest;
use crate::client_session::ClientSessionId;
use crate::club::UserJoinedClubRow;
use crate::db::club::UserClubRow;
use crate::layers::oidc::AdditionalClaims;
use crate::peertube::user::PeerTubeAccountId;
use crate::AppState;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct UserId(i64);

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub oidc_subject: Option<Uuid>,
    pub peertube_account_id: Option<PeerTubeAccountId>,
}

#[derive(Clone, Debug)]
pub struct PublicUserData {
    pub id: UserId,
    pub public_name: String,
    pub peertube_account_id: Option<PeerTubeAccountId>,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct UserRow {
    id: i64,
    oidc_subject: Option<String>,
    peertube_account_id: Option<i64>,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct ExtendedUserRow {
    id: i64,
    peertube_account_id: Option<i64>,
    public_name: String,
}

#[derive(Clone, Debug)]
pub struct UserSearchFilter {
    pub include_guests: bool,
    pub offset: i64,
    pub limit: u16,
    // pub name_fragment: Option<String>,
}

impl UserId {
    pub fn num(&self) -> i64 {
        self.0
    }

    pub(crate) async fn create_new_guest(db: &PgPool) -> Self {
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (oidc_subject) 
            VALUES (null)
            RETURNING id
            "#,
        )
        .fetch_one(db)
        .await
        .expect("Failed to insert new guest user")
        .id;
        UserId(user_id)
    }
}

impl UserId {
    pub(crate) async fn lookup_by_client_secret(
        state: &AppState,
        client_session_id: i64,
        client_session_secret: Uuid,
    ) -> Option<UserId> {
        let client_session = ClientSessionId::authenticate_guest_session(
            &state.pg_db_pool,
            client_session_id,
            &client_session_secret,
        )
        .await?;

        let user_id = sqlx::query!(
            "SELECT user_id FROM client_session WHERE id = $1",
            client_session.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed");

        user_id.map(|record| UserId(record.user_id))
    }
}

impl User {
    pub(crate) async fn lookup(state: &AppState, user_id: UserId) -> Option<User> {
        let record = sqlx::query_as!(
            UserRow,
            "SELECT id, oidc_subject, peertube_account_id FROM users WHERE id = $1",
            user_id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed")?;

        Some(User::from(record))
    }

    pub(crate) async fn lookup_by_client_secret(
        state: &AppState,
        client_session_id: i64,
        client_session_secret: Uuid,
    ) -> Option<User> {
        let user_id =
            UserId::lookup_by_client_secret(state, client_session_id, client_session_secret)
                .await?;

        User::lookup(state, user_id).await
    }

    pub(crate) async fn try_lookup_by_oidc(
        state: &AppState,
        claims: &OidcClaims<axum_oidc::EmptyAdditionalClaims>,
    ) -> Option<User> {
        let subject = claims.subject().as_str();

        let record = sqlx::query_as!(
            UserRow,
            "SELECT id, oidc_subject, peertube_account_id FROM users WHERE oidc_subject = $1",
            subject
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed")?;

        Some(User::from(record))
    }

    pub(crate) async fn lookup_by_oidc_or_create(
        state: &AppState,
        claims: OidcClaims<AdditionalClaims>,
    ) -> User {
        if let Some(maybe_user) = Self::try_lookup_by_oidc(state, &claims).await {
            return maybe_user;
        }

        // Lazy user row creation in DB
        let subject = claims.subject().as_str();

        let id = sqlx::query!(
            r#"
        INSERT INTO users (oidc_subject) 
        VALUES ($1)
        RETURNING id
        "#,
            subject
        )
        .fetch_one(&state.pg_db_pool)
        .await
        .expect("Failed to insert new user")
        .id;

        Self::new(id, Some(subject), None)
    }

    pub(crate) async fn add_oidc(&mut self, state: &AppState, sub: Uuid) -> sqlx::Result<()> {
        assert!(
            self.oidc_subject.is_none(),
            "can't overwrite existing OIDC: {:?}",
            self.oidc_subject
        );

        let result = sqlx::query!(
            r#"UPDATE users SET oidc_subject = $1 WHERE id = $2"#,
            sub.to_string(),
            self.id.num()
        )
        .execute(&state.pg_db_pool)
        .await?;

        if result.rows_affected() != 1 {
            tracing::error!("add_oidc affected {} rows", result.rows_affected());
        }

        self.oidc_subject = Some(sub);
        Ok(())
    }

    pub(crate) async fn add_peertube_id(
        &mut self,
        state: &AppState,
        peertube_id: PeerTubeAccountId,
    ) -> sqlx::Result<()> {
        assert!(
            self.peertube_account_id.is_none(),
            "can't overwrite existing PeerTube ID: {:?}",
            self.peertube_account_id
        );

        let result = sqlx::query!(
            r#"UPDATE users SET peertube_account_id = $1 WHERE id = $2"#,
            peertube_id.0,
            self.id.num()
        )
        .execute(&state.pg_db_pool)
        .await?;

        if result.rows_affected() != 1 {
            tracing::error!("add_peertube_id affected {} rows", result.rows_affected());
        }

        self.peertube_account_id = Some(peertube_id);
        Ok(())
    }

    pub(crate) async fn lookup_public_info(
        state: &AppState,
        user_id: UserId,
    ) -> Option<PublicUserData> {
        let record = sqlx::query_as!(
            ExtendedUserRow,
            r#"
                SELECT u.id, u.peertube_account_id, um.key_value AS public_name
                FROM users u
                JOIN user_meta um on um.user_id = u.id
                WHERE u.id = $1
                    AND um.key_name = 's:publicName'
            "#,
            user_id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed")?;

        Some(PublicUserData::from(record))
    }

    pub(crate) async fn list(
        state: &AppState,
        filter: &UserSearchFilter,
    ) -> sqlx::Result<Vec<PublicUserData>> {
        let rows = if filter.include_guests {
            sqlx::query_as!(
                ExtendedUserRow,
                r#"
                SELECT u.id, u.peertube_account_id, um.key_value AS public_name
                FROM users u
                JOIN user_meta um on um.user_id = u.id
                WHERE um.key_name = 's:publicName'
                ORDER BY um.user_id LIMIT $1 OFFSET $2
                "#,
                filter.limit as i32,
                filter.offset,
            )
            .fetch_all(&state.pg_db_pool)
            .await?
        } else {
            sqlx::query_as!(
                ExtendedUserRow,
                r#"
                SELECT u.id, u.peertube_account_id, um.key_value AS public_name
                FROM users u
                JOIN user_meta um on um.user_id = u.id
                WHERE um.key_name = 's:publicName'
                    AND oidc_subject IS NOT NULL
                ORDER BY u.id LIMIT $1 OFFSET $2
                "#,
                filter.limit as i32,
                filter.offset,
            )
            .fetch_all(&state.pg_db_pool)
            .await?
        };

        let users = rows.into_iter().map(PublicUserData::from).collect();
        Ok(users)
    }

    fn new(id: i64, subject: Option<&str>, peertube_account_id: Option<PeerTubeAccountId>) -> User {
        let oidc_subject =
            subject.map(|sub| Uuid::parse_str(sub).expect("sub must be a valid UUID"));
        User {
            id: UserId(id),
            oidc_subject,
            peertube_account_id,
        }
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserClubRow {
    pub fn user_id(&self) -> UserId {
        UserId(self.user_id)
    }
}

impl UserJoinedClubRow {
    pub fn user_id(&self) -> UserId {
        UserId(self.user_id)
    }
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User::new(
            row.id,
            row.oidc_subject.as_deref(),
            row.peertube_account_id.map(PeerTubeAccountId),
        )
    }
}

impl From<ExtendedUserRow> for PublicUserData {
    fn from(row: ExtendedUserRow) -> Self {
        PublicUserData {
            id: UserId(row.id),
            public_name: row.public_name,
            peertube_account_id: row.peertube_account_id.map(PeerTubeAccountId),
        }
    }
}

impl AddClubMemberRequest {
    pub(crate) fn user_id(&self) -> UserId {
        UserId(self.user_id)
    }
}
