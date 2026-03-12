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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::apply_migrations;
    use sqlx::PgPool;

    /// Insert a guest user; return the UserId.
    async fn setup_guest_user(pool: &PgPool) -> UserId {
        UserId::create_new_guest(pool).await
    }

    /// Insert a guest user and create a client session for them; return both IDs and the secret.
    async fn setup_client_session(
        pool: &PgPool,
    ) -> (UserId, i64, Uuid) {
        let user_id = setup_guest_user(pool).await;
        let secret = Uuid::new_v4();

        let session_id: i64 = sqlx::query_scalar(
            "INSERT INTO client_session (user_id, client_session_secret) VALUES ($1, $2) RETURNING id",
        )
        .bind(user_id.num())
        .bind(secret)
        .fetch_one(pool)
        .await
        .expect("failed to insert test client session");

        (user_id, session_id, secret)
    }

    // ── ClientSessionId::authenticate_guest_session ──────────────────────────

    #[sqlx::test]
    async fn authenticate_guest_session_succeeds_with_valid_secret(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let (_user_id, session_id, secret) = setup_client_session(&pool).await;

        let result = ClientSessionId::authenticate_guest_session(&pool, session_id, &secret).await;

        assert!(result.is_some(), "authentication should succeed with valid secret");
        let authenticated_session = result.unwrap();
        assert_eq!(authenticated_session.num(), session_id);
        Ok(())
    }

    #[sqlx::test]
    async fn authenticate_guest_session_fails_with_invalid_secret(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let (_user_id, session_id, _secret) = setup_client_session(&pool).await;
        let wrong_secret = Uuid::new_v4();

        let result = ClientSessionId::authenticate_guest_session(&pool, session_id, &wrong_secret).await;

        assert!(result.is_none(), "authentication should fail with invalid secret");
        Ok(())
    }

    #[sqlx::test]
    async fn authenticate_guest_session_fails_with_nonexistent_session(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let secret = Uuid::new_v4();
        let nonexistent_session_id = i64::MAX;

        let result = ClientSessionId::authenticate_guest_session(&pool, nonexistent_session_id, &secret).await;

        assert!(result.is_none(), "authentication should fail for nonexistent session");
        Ok(())
    }

    #[sqlx::test]
    async fn authenticate_guest_session_handles_multiple_sessions_isolation(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let (_user_a, session_a_id, secret_a) = setup_client_session(&pool).await;
        let (_user_b, session_b_id, secret_b) = setup_client_session(&pool).await;

        // Session A should authenticate with its own secret only
        let result_a_with_a = ClientSessionId::authenticate_guest_session(&pool, session_a_id, &secret_a).await;
        let result_a_with_b = ClientSessionId::authenticate_guest_session(&pool, session_a_id, &secret_b).await;

        // Session B should authenticate with its own secret only
        let result_b_with_b = ClientSessionId::authenticate_guest_session(&pool, session_b_id, &secret_b).await;
        let result_b_with_a = ClientSessionId::authenticate_guest_session(&pool, session_b_id, &secret_a).await;

        assert!(result_a_with_a.is_some(), "session A should authenticate with its secret");
        assert!(result_a_with_b.is_none(), "session A should not authenticate with session B's secret");
        assert!(result_b_with_b.is_some(), "session B should authenticate with its secret");
        assert!(result_b_with_a.is_none(), "session B should not authenticate with session A's secret");
        Ok(())
    }

    #[sqlx::test]
    async fn authenticate_guest_session_returns_correct_session_id(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let (_user_id, session_id, secret) = setup_client_session(&pool).await;

        let authenticated = ClientSessionId::authenticate_guest_session(&pool, session_id, &secret)
            .await
            .expect("authentication should succeed");

        assert_eq!(authenticated.num(), session_id, "returned session ID should match the input");
        Ok(())
    }

    // ── ClientSessionId::transfer_client_sessions ──────────────────────────

    #[sqlx::test]
    async fn transfer_client_sessions_moves_sessions_to_target_user(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let source_user = setup_guest_user(&pool).await;
        let target_user = setup_guest_user(&pool).await;

        // Create multiple sessions for source user
        let _session_a: i64 = sqlx::query_scalar(
            "INSERT INTO client_session (user_id, client_session_secret) VALUES ($1, $2) RETURNING id",
        )
        .bind(source_user.num())
        .bind(Uuid::new_v4())
        .fetch_one(&pool)
        .await?;

        let _session_b: i64 = sqlx::query_scalar(
            "INSERT INTO client_session (user_id, client_session_secret) VALUES ($1, $2) RETURNING id",
        )
        .bind(source_user.num())
        .bind(Uuid::new_v4())
        .fetch_one(&pool)
        .await?;

        // Transfer sessions
        ClientSessionId::transfer_client_sessions(&pool, source_user, target_user).await?;

        // Verify sessions are now owned by target user
        let source_session_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM client_session WHERE user_id = $1",
        )
        .bind(source_user.num())
        .fetch_one(&pool)
        .await?;

        let target_session_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM client_session WHERE user_id = $1",
        )
        .bind(target_user.num())
        .fetch_one(&pool)
        .await?;

        assert_eq!(
            source_session_count, 0,
            "source user should have no sessions after transfer"
        );
        assert_eq!(
            target_session_count, 2,
            "target user should have all transferred sessions"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn transfer_client_sessions_with_no_sessions_succeeds(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let source_user = setup_guest_user(&pool).await;
        let target_user = setup_guest_user(&pool).await;

        // Transfer should succeed even if source user has no sessions
        let result = ClientSessionId::transfer_client_sessions(&pool, source_user, target_user).await;

        assert!(
            result.is_ok(),
            "transfer should succeed even with no sessions to transfer"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn transfer_client_sessions_preserves_session_secrets(pool: PgPool) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let source_user = setup_guest_user(&pool).await;
        let target_user = setup_guest_user(&pool).await;
        let secret = Uuid::new_v4();

        let session_id: i64 = sqlx::query_scalar(
            "INSERT INTO client_session (user_id, client_session_secret) VALUES ($1, $2) RETURNING id",
        )
        .bind(source_user.num())
        .bind(secret)
        .fetch_one(&pool)
        .await?;

        // Transfer sessions
        ClientSessionId::transfer_client_sessions(&pool, source_user, target_user).await?;

        // Verify the session can still be authenticated with the same secret
        let authenticated = ClientSessionId::authenticate_guest_session(&pool, session_id, &secret)
            .await
            .expect("session should still be authenticatable after transfer");

        assert_eq!(
            authenticated.num(),
            session_id,
            "session should retain its ID after transfer"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn transfer_client_sessions_partial_transfer_only_affects_source(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let source_user = setup_guest_user(&pool).await;
        let target_user = setup_guest_user(&pool).await;
        let unrelated_user = setup_guest_user(&pool).await;

        // Create sessions for multiple users
        let _session_source: i64 = sqlx::query_scalar(
            "INSERT INTO client_session (user_id, client_session_secret) VALUES ($1, $2) RETURNING id",
        )
        .bind(source_user.num())
        .bind(Uuid::new_v4())
        .fetch_one(&pool)
        .await?;

        let _session_unrelated: i64 = sqlx::query_scalar(
            "INSERT INTO client_session (user_id, client_session_secret) VALUES ($1, $2) RETURNING id",
        )
        .bind(unrelated_user.num())
        .bind(Uuid::new_v4())
        .fetch_one(&pool)
        .await?;

        // Transfer sessions
        ClientSessionId::transfer_client_sessions(&pool, source_user, target_user).await?;

        // Verify unrelated user's sessions are not affected
        let unrelated_session_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM client_session WHERE user_id = $1",
        )
        .bind(unrelated_user.num())
        .fetch_one(&pool)
        .await?;

        assert_eq!(
            unrelated_session_count, 1,
            "unrelated user's sessions should not be affected"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn transfer_client_sessions_handles_empty_transfer_silently(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        apply_migrations(&pool).await;
        let source_user = setup_guest_user(&pool).await;
        let target_user = setup_guest_user(&pool).await;

        // Transfer with no source sessions should complete without error
        let result = ClientSessionId::transfer_client_sessions(&pool, source_user, target_user).await;

        assert!(result.is_ok(), "transfer of no sessions should succeed");

        // Verify target user still has no sessions
        let target_session_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM client_session WHERE user_id = $1",
        )
        .bind(target_user.num())
        .fetch_one(&pool)
        .await?;

        assert_eq!(target_session_count, 0, "target user should still have no sessions");
        Ok(())
    }

}
