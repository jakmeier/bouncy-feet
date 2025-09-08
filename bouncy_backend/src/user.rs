use axum_oidc::OidcClaims;
use sqlx::PgPool;
use uuid::Uuid;

use crate::client_session::ClientSessionId;
use crate::layers::oidc::AdditionalClaims;
use crate::AppState;

#[derive(Clone)]
pub struct UserId(i64);

#[derive(Clone)]
pub struct User {
    pub id: UserId,
    pub oidc_subject: Option<Uuid>,
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
        let maybe_user = sqlx::query!(
            "SELECT id, oidc_subject FROM users WHERE id = $1",
            user_id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed");

        maybe_user.map(|record| User::new(record.id, record.oidc_subject.as_deref()))
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

    pub(crate) async fn lookup_by_oidc(
        state: &AppState,
        claims: OidcClaims<AdditionalClaims>,
    ) -> User {
        let subject = claims.subject().as_str();

        let maybe_user = sqlx::query!(
            "SELECT id, oidc_subject FROM users WHERE oidc_subject = $1",
            subject
        )
        .fetch_optional(&state.pg_db_pool)
        .await
        .expect("DB query failed");

        // Lazy user row creation in DB
        let id = match maybe_user {
            Some(user) => user.id,
            None => {
                sqlx::query!(
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
                .id
            }
        };
        Self::new(id, Some(subject))
    }

    fn new(id: i64, subject: Option<&str>) -> User {
        let oidc_subject =
            subject.map(|sub| Uuid::parse_str(sub).expect("sub must be a valid UUID"));
        User {
            id: UserId(id),
            oidc_subject: oidc_subject,
        }
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
