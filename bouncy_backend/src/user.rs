use axum_oidc::OidcClaims;
use sqlx::PgPool;
use uuid::Uuid;

use crate::client_session::ClientSessionId;
use crate::layers::oidc::AdditionalClaims;
use crate::AppState;

#[derive(Clone)]
pub struct UserId(i64);

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
    pub(crate) async fn user_lookup_by_client_secret(
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

    pub(crate) async fn user_lookup_by_oidc(
        state: &AppState,
        claims: OidcClaims<AdditionalClaims>,
    ) -> UserId {
        let subject = claims.subject().as_str();

        let maybe_user = sqlx::query!("SELECT id FROM users WHERE oidc_subject = $1", subject)
            .fetch_optional(&state.pg_db_pool)
            .await
            .expect("DB query failed");

        // Lazy user row creation in DB
        let user_id = match maybe_user {
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
        UserId(user_id)
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
