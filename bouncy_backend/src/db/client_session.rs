use crate::user::UserId;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Clone)]
/// Authenticated client session
pub struct ClientSessionId(i64);

impl ClientSessionId {
    // Authenticate a guest client session by its secret.
    pub async fn authenticate_guest_session(
        db_pool: &PgPool,
        client_session_id: i64,
        secret: &Uuid,
    ) -> Option<Self> {
        match sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM client_session
                WHERE id = $1 AND client_session_secret = $2
            )
            "#,
            client_session_id,
            secret
        )
        .fetch_one(db_pool)
        .await
        {
            Ok(Some(true)) => Some(ClientSessionId(client_session_id)),
            Ok(Some(false)) => {
                info!(
                    client_session_id,
                    "Guest session authentication failed (invalid secret)"
                );

                None
            }
            Ok(None) => {
                error!(client_session_id, "Database returned None for some reason");
                None
            }
            Err(e) => {
                error!(
                    client_session_id,
                    error = %e,
                    "Database error during guest session authentication"
                );
                None
            }
        }
    }

    pub async fn transfer_client_sessions(
        pool: &PgPool,
        from: UserId,
        to: UserId,
    ) -> sqlx::Result<()> {
        let result = sqlx::query!(
            r#"
                UPDATE client_session
                SET user_id = $1
                WHERE user_id = $2
            "#,
            to.num(),
            from.num()
        )
        .execute(pool)
        .await?;

        let rows_affected = result.rows_affected();
        info!(
            "Transferred {} client sessions from {} to {}",
            rows_affected, from, to,
        );
        Ok(())
    }
}

impl ClientSessionId {
    pub fn num(&self) -> i64 {
        self.0
    }
}
