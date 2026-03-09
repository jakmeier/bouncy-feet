use axum_oidc::OidcClaims;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api_endoints::club::AddClubMemberRequest;
use crate::client_session::ClientSessionId;
use crate::club::UserJoinedClubRow;
use crate::combo::ComboRow;
use crate::db::club::UserClubRow;
use crate::layers::oidc::AdditionalClaims;
use crate::peertube::user::{PeerTubeAccountId, PeerTubeHandle};
use crate::AppState;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct UserId(i64);

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub oidc_subject: Option<Uuid>,
    pub peertube_account_id: Option<PeerTubeAccountId>,
    pub peertube_handle: Option<PeerTubeHandle>,
}

#[derive(Clone, Debug)]
pub struct PublicUserData {
    pub id: UserId,
    pub public_name: String,
    pub peertube_handle: Option<PeerTubeHandle>,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct UserRow {
    id: i64,
    oidc_subject: Option<String>,
    peertube_account_id: Option<i64>,
    peertube_handle: Option<String>,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct ExtendedUserRow {
    id: i64,
    peertube_handle: Option<String>,
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
            "SELECT id, oidc_subject, peertube_account_id, peertube_handle FROM users WHERE id = $1",
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
            "SELECT id, oidc_subject, peertube_account_id, peertube_handle FROM users WHERE oidc_subject = $1",
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

        Self::new(id, Some(subject), None, None)
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

    pub(crate) async fn set_peertube_handle(
        &mut self,
        state: &AppState,
        peertube_handle: PeerTubeHandle,
    ) -> sqlx::Result<()> {
        let result = sqlx::query!(
            r#"UPDATE users SET peertube_handle = $1 WHERE id = $2"#,
            peertube_handle.0,
            self.id.num()
        )
        .execute(&state.pg_db_pool)
        .await?;

        if result.rows_affected() != 1 {
            tracing::error!(
                "set_peertube_handle affected {} rows",
                result.rows_affected()
            );
        }

        self.peertube_handle = Some(peertube_handle);
        Ok(())
    }

    pub(crate) async fn lookup_public_info(
        state: &AppState,
        user_id: UserId,
    ) -> Option<PublicUserData> {
        let record = sqlx::query_as!(
            ExtendedUserRow,
            r#"
                SELECT u.id, u.peertube_handle, um.key_value AS public_name
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
                SELECT u.id, u.peertube_handle, um.key_value AS public_name
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
                SELECT u.id, u.peertube_handle, um.key_value AS public_name
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

    fn new(
        id: i64,
        subject: Option<&str>,
        peertube_account_id: Option<PeerTubeAccountId>,
        peertube_handle: Option<PeerTubeHandle>,
    ) -> User {
        let oidc_subject =
            subject.map(|sub| Uuid::parse_str(sub).expect("sub must be a valid UUID"));
        User {
            id: UserId(id),
            oidc_subject,
            peertube_account_id,
            peertube_handle,
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

impl ComboRow {
    pub(crate) fn user_id(&self) -> UserId {
        UserId(self.user_id)
    }
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User::new(
            row.id,
            row.oidc_subject.as_deref(),
            row.peertube_account_id.map(PeerTubeAccountId),
            row.peertube_handle.map(PeerTubeHandle),
        )
    }
}

impl From<ExtendedUserRow> for PublicUserData {
    fn from(row: ExtendedUserRow) -> Self {
        PublicUserData {
            id: UserId(row.id),
            public_name: row.public_name,
            peertube_handle: row.peertube_handle.map(PeerTubeHandle),
        }
    }
}

impl AddClubMemberRequest {
    pub(crate) fn user_id(&self) -> UserId {
        UserId(self.user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_endoints::auth::KeycloakClientConfig;
    use crate::cache::DataCache;
    use crate::peertube::system_user::PeerTubeSystemUser;
    use sqlx::PgPool;
    use std::sync::Arc;
    use url::Url;

    async fn apply_migrations(pool: &sqlx::PgPool) {
        sqlx::migrate::Migrator::new(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("db_migrations")
        )
        .await
        .expect("failed to build migrator")
        .run(pool)
        .await
        .expect("failed to run migrations");
    }

    /// Build a minimal AppState for testing.
    /// Only the `pg_db_pool` field is meaningful; all other fields use dummy values.
    fn make_test_state(pool: PgPool) -> crate::AppState {
        crate::AppState {
            app_url: "http://localhost:3000".parse::<Url>().unwrap(),
            api_url: "http://localhost:4000".parse::<Url>().unwrap(),
            peertube_url: "http://localhost:9000".parse::<Url>().unwrap(),
            pg_db_pool: pool,
            http_client: reqwest::Client::new(),
            peertube_client_config: Arc::new(tokio::sync::RwLock::new(None)),
            kc_config: KeycloakClientConfig {
                client_id: "test-client".to_string(),
                client_secret: "test-secret".to_string(),
                registration_url: "http://localhost:8080/register"
                    .parse::<Url>()
                    .unwrap(),
                logout_url: "http://localhost:8080/logout".parse::<Url>().unwrap(),
            },
            system_user: PeerTubeSystemUser::new(
                "system_user".to_string(),
                "password".to_string(),
            ),
            data_cache: DataCache::default(),
        }
    }

    // ── UserId::create_new_guest ─────────────────────────────────────────────

    #[sqlx::test]
    async fn create_guest_user_returns_valid_id(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let user_id = UserId::create_new_guest(&pool).await;
        assert!(
            user_id.num() > 0,
            "newly created user should have a positive id"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn create_multiple_guests_have_distinct_ids(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let id1 = UserId::create_new_guest(&pool).await;
        let id2 = UserId::create_new_guest(&pool).await;
        assert_ne!(
            id1.num(),
            id2.num(),
            "each guest creation should yield a unique id"
        );
        Ok(())
    }

    // ── User::lookup ─────────────────────────────────────────────────────────

    #[sqlx::test]
    async fn lookup_existing_user_returns_some(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);
        let user_id = UserId::create_new_guest(&state.pg_db_pool).await;

        let user = User::lookup(&state, user_id).await;

        assert!(user.is_some(), "should find the user that was just created");
        let user = user.unwrap();
        assert_eq!(user.id.num(), user_id.num());
        assert!(
            user.oidc_subject.is_none(),
            "guest user should have no OIDC subject"
        );
        assert!(
            user.peertube_account_id.is_none(),
            "fresh user should have no PeerTube account id"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn lookup_nonexistent_user_returns_none(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let state = make_test_state(pool);

        // Use an id that cannot exist in an empty (freshly migrated) database.
        let missing = User::lookup(&state, UserId(i64::MAX)).await;
        assert!(
            missing.is_none(),
            "looking up a non-existent user should return None"
        );
        Ok(())
    }

    // ── User::lookup_by_oidc_or_create ───────────────────────────────────────

    #[sqlx::test]
    async fn oidc_user_is_created_on_first_login(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        // We test the underlying INSERT directly because constructing a full
        // OidcClaims is not feasible without an OIDC server.  This mirrors what
        // lookup_by_oidc_or_create does internally.
        let subject = Uuid::new_v4().to_string();

        // Use the dynamic (non-macro) query to avoid compile-time DB checks
        // in offline mode.  The correctness of the SQL is validated at
        // runtime by the sqlx::test framework.
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO users (oidc_subject) VALUES ($1) RETURNING id",
        )
        .bind(&subject)
        .fetch_one(&pool)
        .await?;

        let state = make_test_state(pool);
        let user = User::lookup(&state, UserId(id)).await;

        assert!(user.is_some());
        let user = user.unwrap();
        let stored_sub = user
            .oidc_subject
            .expect("OIDC subject must be stored");
        assert_eq!(
            stored_sub.to_string(),
            subject,
            "stored OIDC subject must match the one that was inserted"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn oidc_subject_uniqueness_is_enforced(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let subject = Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO users (oidc_subject) VALUES ($1)")
            .bind(&subject)
            .execute(&pool)
            .await?;

        // A second insert with the same subject must fail due to the UNIQUE
        // constraint on users.oidc_subject.
        let result = sqlx::query("INSERT INTO users (oidc_subject) VALUES ($1)")
            .bind(&subject)
            .execute(&pool)
            .await;

        assert!(
            result.is_err(),
            "inserting a duplicate OIDC subject should violate the UNIQUE constraint"
        );
        Ok(())
    }

    // ── UserId::num / Display ─────────────────────────────────────────────────

    #[test]
    fn user_id_display_shows_inner_value() {
        let id = UserId(42);
        assert_eq!(id.to_string(), "42");
        assert_eq!(id.num(), 42);
    }
}
