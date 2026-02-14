use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::PgPool;
use tower_sessions::cookie::time::{Duration, OffsetDateTime};
use tower_sessions::cookie::SameSite;
use tower_sessions::session::Record;
use tower_sessions::session_store::{Error, Result};
use tower_sessions::{ExpiredDeletion, Expiry, MemoryStore, SessionManagerLayer, SessionStore};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PgSessionStore {
    pool: PgPool,
}

pub fn pg_backed_cookie_session_layer(
    url: &Url,
    pool: PgPool,
) -> SessionManagerLayer<PgSessionStore> {
    cookie_session_layer(url, PgSessionStore { pool })
}

#[allow(dead_code)]
/// Can be used for testing but not good for production.
///
/// The main problem with this in production: Restarting the server breaks all
/// existing sessions. And since there is no clean-up code, the sessions remain
/// broken.
pub fn in_memory_cookie_session_layer(url: &Url) -> SessionManagerLayer<MemoryStore> {
    cookie_session_layer(url, MemoryStore::default())
}

fn cookie_session_layer<STORE: SessionStore>(
    url: &Url,
    session_store: STORE,
) -> SessionManagerLayer<STORE> {
    let domain = url.domain().expect("invalid url").to_string();

    SessionManagerLayer::new(session_store)
        // js access is not needed
        .with_http_only(true)
        .with_domain(domain)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)))
        .with_always_save(true)
        // absolutely don't leak the session id, authentication relies on it!
        .with_secure(true) // dev?
        // Same-site should work within the same registerable domain, e.g. bouncy-feet.ch
        // https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-cookie-same-site-00#section-2
        // But when accessing from a web view in a PWA, Strict or Lax sadly doesn't work.
        .with_same_site(SameSite::None)
}

#[async_trait]
impl SessionStore for PgSessionStore {
    async fn create(&self, record: &mut Record) -> Result<()> {
        let (id, expiry_date, data) = extract_column_values(record)?;
        // TODO: consider retrying on collision
        sqlx::query!(
            r#"
            INSERT INTO http_sessions (id, expiry_date, session_data) 
            VALUES($1, $2, $3);
            "#,
            id,
            expiry_date.naive_utc(),
            data,
        )
        .execute(&self.pool)
        .await
        .map_err(to_tower_session_error)?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> Result<()> {
        let (id, expiry_date, data) = extract_column_values(record)?;
        sqlx::query!(
            r#"
            UPDATE http_sessions
            SET (expiry_date, session_data) = ($2, $3)
            WHERE id = $1;
            "#,
            id,
            expiry_date.naive_utc(),
            data
        )
        .execute(&self.pool)
        .await
        .map_err(to_tower_session_error)?;
        Ok(())
    }

    async fn load(&self, session_id: &tower_sessions::session::Id) -> Result<Option<Record>> {
        let id = to_uuid(session_id);
        let row = sqlx::query!(
            r#"
            SELECT expiry_date, session_data
            FROM http_sessions
            WHERE id = $1;
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(to_tower_session_error)?;

        if let Some(row) = row {
            let data: HashMap<String, serde_json::Value> = if let Some(raw_data) = row.session_data
            {
                serde_json::from_value(raw_data).map_err(|e| Error::Decode(e.to_string()))?
            } else {
                HashMap::new()
            };

            Ok(Some(Record {
                id: *session_id,
                data,
                expiry_date: OffsetDateTime::from_unix_timestamp(
                    row.expiry_date.and_utc().timestamp(),
                )
                .map_err(|e| Error::Decode(e.to_string()))?,
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &tower_sessions::session::Id) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM http_sessions
            WHERE id = $1;
            "#,
            to_uuid(session_id),
        )
        .execute(&self.pool)
        .await
        .map_err(to_tower_session_error)?;

        Ok(())
    }
}

#[async_trait]
impl ExpiredDeletion for PgSessionStore {
    async fn delete_expired(&self) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM http_sessions
            WHERE expiry_date < (now() at time zone 'utc');
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(to_tower_session_error)?;
        Ok(())
    }
}

fn to_uuid(id: &tower_sessions::session::Id) -> Uuid {
    Uuid::from_bytes(id.0.to_le_bytes())
}

fn extract_column_values(
    record: &Record,
) -> Result<(Uuid, chrono::DateTime<chrono::Utc>, serde_json::Value)> {
    let id = to_uuid(&record.id);
    // Convert to chrono::DateTime<Utc> to avoid using time and chrono features of sqlx
    let timestamp = record.expiry_date.unix_timestamp();
    let expiry_date = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp, 0)
        .expect("session must use valid expiry date");
    let data = serde_json::to_value(&record.data).map_err(|e| Error::Encode(e.to_string()))?;
    Ok((id, expiry_date, data))
}

fn to_tower_session_error(e: sqlx::Error) -> Error {
    Error::Backend(e.to_string())
}
