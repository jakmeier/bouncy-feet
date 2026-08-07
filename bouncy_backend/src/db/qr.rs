use crate::db::user::UserId;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Transaction};
use thiserror::Error;
use url::Url;

/// A validated opaque code as printed in a QR URL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct QrCode(String);

/// The SHA-256 identifier used for all public QR lookups and database keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct QrHash([u8; 32]);

#[derive(Debug, Error, Eq, PartialEq)]
pub(crate) enum QrCodeError {
    #[error("QR codes must contain 1 to 1024 ASCII letters, digits, hyphens, or underscores")]
    InvalidCode,
    #[error("QR hash must be exactly 64 lowercase hexadecimal characters")]
    InvalidHash,
}

impl QrCode {
    pub(crate) fn parse(code: String) -> Result<Self, QrCodeError> {
        if is_valid_qr_code(&code) {
            Ok(Self(code))
        } else {
            Err(QrCodeError::InvalidCode)
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn hash(&self) -> QrHash {
        QrHash::from_code(self)
    }
}

/// Validate without trimming, case-folding, decoding, or otherwise normalizing.
pub(crate) fn is_valid_qr_code(code: &str) -> bool {
    let bytes = code.as_bytes();
    (1..=1024).contains(&bytes.len())
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(*byte, b'-' | b'_'))
}

impl QrHash {
    pub(crate) fn from_code(code: &QrCode) -> Self {
        let digest = Sha256::digest(code.as_str().as_bytes());
        let mut bytes = [0; 32];
        bytes.copy_from_slice(&digest);
        Self(bytes)
    }

    pub(crate) fn parse_lower_hex(value: &str) -> Result<Self, QrCodeError> {
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            return Err(QrCodeError::InvalidHash);
        }

        let mut bytes = [0; 32];
        hex::decode_to_slice(value, &mut bytes).map_err(|_| QrCodeError::InvalidHash)?;
        Ok(Self(bytes))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub(crate) fn to_lower_hex(self) -> String {
        hex::encode(self.0)
    }

    fn advisory_lock_key(self) -> i64 {
        i64::from_be_bytes(
            self.0[..8]
                .try_into()
                .expect("SHA-256 prefix is eight bytes"),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TargetUrl(String);

#[derive(Debug, Error, Eq, PartialEq)]
pub(crate) enum TargetUrlError {
    #[error("target URL exceeds 2048 characters")]
    TooLong,
    #[error("target URL must be an absolute URL")]
    NotAbsolute,
    #[error("target URL must use HTTPS")]
    InsecureScheme,
    #[error("target URL must not contain credentials")]
    Credentials,
    #[error("target URL origin is not trusted")]
    UntrustedOrigin,
    #[error("target URL must not point to the QR forwarding route")]
    QrLoop,
}

impl TargetUrl {
    /// Validate a target before an admin mapping event is inserted.
    pub(crate) fn parse(
        value: String,
        app_origin: &Url,
        trusted_origins: &[Url],
        allow_localhost_http: bool,
    ) -> Result<Self, TargetUrlError> {
        if value.chars().count() > 2048 {
            return Err(TargetUrlError::TooLong);
        }
        let url = Url::parse(&value).map_err(|_| TargetUrlError::NotAbsolute)?;
        if url.cannot_be_a_base() || url.host().is_none() {
            return Err(TargetUrlError::NotAbsolute);
        }

        let localhost_http = allow_localhost_http
            && url.scheme() == "http"
            && matches!(
                url.host_str(),
                Some("localhost") | Some("127.0.0.1") | Some("::1")
            );
        if url.scheme() != "https" && !localhost_http {
            return Err(TargetUrlError::InsecureScheme);
        }
        if !url.username().is_empty() || url.password().is_some() {
            return Err(TargetUrlError::Credentials);
        }

        let origin = url.origin();
        if origin != app_origin.origin()
            && !trusted_origins
                .iter()
                .any(|allowed| allowed.origin() == origin)
        {
            return Err(TargetUrlError::UntrustedOrigin);
        }
        if url.path() == "/qr" || url.path().starts_with("/qr/") {
            return Err(TargetUrlError::QrLoop);
        }

        Ok(Self(value))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct CurrentMapping {
    pub(crate) event_id: i64,
    pub(crate) target_url: String,
    pub(crate) is_disabled: bool,
}

#[derive(sqlx::FromRow)]
struct CurrentMappingState {
    target_url: String,
    is_disabled: bool,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub(crate) struct MappingEvent {
    pub(crate) id: i64,
    pub(crate) target_url: String,
    pub(crate) is_disabled: bool,
    pub(crate) changed_at: DateTime<Utc>,
    pub(crate) changed_by: i64,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub(crate) struct MappingHistoryEvent {
    pub(crate) id: i64,
    pub(crate) target_url: String,
    pub(crate) is_disabled: bool,
    pub(crate) changed_at: DateTime<Utc>,
    pub(crate) active_until: Option<DateTime<Utc>>,
    pub(crate) changed_by: i64,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub(crate) struct QrCodeSummary {
    pub(crate) code_hash: String,
    pub(crate) code: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) created_by: i64,
    pub(crate) current_event_id: Option<i64>,
    pub(crate) current_target_url: Option<String>,
    pub(crate) current_is_disabled: Option<bool>,
    pub(crate) current_changed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub(crate) struct QrCodeDetail {
    pub(crate) code_hash: String,
    pub(crate) code: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) created_by: i64,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub(crate) struct AccessMinuteBucket {
    pub(crate) mapping_event_id: i64,
    pub(crate) target_url: String,
    pub(crate) bucket_start: DateTime<Utc>,
    pub(crate) access_count: i64,
}

#[derive(Debug, Error)]
pub(crate) enum QrAdminDbError {
    #[error("QR code is already configured")]
    AlreadyConfigured,
    #[error("SHA-256 collision between distinct QR codes")]
    HashCollision,
    #[error("QR code was not found")]
    NotFound,
    #[error("mapping is already in the requested state")]
    NoChange,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// Create the immutable code row and first active mapping atomically. The
/// advisory lock closes the absent-row race. Later edits use a row lock.
pub(crate) async fn create_code_and_mapping(
    db: &PgPool,
    code: &QrCode,
    target_url: &TargetUrl,
    changed_by: UserId,
) -> Result<MappingEvent, QrAdminDbError> {
    let hash = code.hash();
    let mut tx = db.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(hash.advisory_lock_key())
        .execute(&mut *tx)
        .await?;

    let existing: Option<String> =
        sqlx::query_scalar("SELECT code FROM qr_codes WHERE code_hash = $1 FOR UPDATE")
            .bind(hash.as_bytes().as_slice())
            .fetch_optional(&mut *tx)
            .await?;
    if let Some(existing) = existing {
        return if existing == code.as_str() {
            Err(QrAdminDbError::AlreadyConfigured)
        } else {
            Err(QrAdminDbError::HashCollision)
        };
    }

    sqlx::query("INSERT INTO qr_codes (code_hash, code, created_by) VALUES ($1, $2, $3)")
        .bind(hash.as_bytes().as_slice())
        .bind(code.as_str())
        .bind(changed_by.num())
        .execute(&mut *tx)
        .await?;
    let event =
        insert_mapping_event_tx(&mut tx, &hash, target_url.as_str(), false, changed_by).await?;
    tx.commit().await?;
    Ok(event)
}

pub(crate) async fn append_mapping(
    db: &PgPool,
    hash: &QrHash,
    target_url: &TargetUrl,
    changed_by: UserId,
) -> Result<MappingEvent, QrAdminDbError> {
    let mut tx = db.begin().await?;
    lock_code(&mut tx, hash).await?;
    let current = current_mapping_state_tx(&mut tx, hash).await?;
    if matches!(current, Some(CurrentMappingState { target_url: current_target, is_disabled: false }) if current_target == target_url.as_str())
    {
        return Err(QrAdminDbError::NoChange);
    }
    let event =
        insert_mapping_event_tx(&mut tx, hash, target_url.as_str(), false, changed_by).await?;
    tx.commit().await?;
    Ok(event)
}

pub(crate) async fn disable_mapping(
    db: &PgPool,
    hash: &QrHash,
    changed_by: UserId,
) -> Result<MappingEvent, QrAdminDbError> {
    let mut tx = db.begin().await?;
    lock_code(&mut tx, hash).await?;
    let Some(current) = current_mapping_state_tx(&mut tx, hash).await? else {
        return Err(QrAdminDbError::NoChange);
    };
    if current.is_disabled {
        return Err(QrAdminDbError::NoChange);
    }
    let event =
        insert_mapping_event_tx(&mut tx, hash, &current.target_url, true, changed_by).await?;
    tx.commit().await?;
    Ok(event)
}

async fn lock_code(
    tx: &mut Transaction<'_, Postgres>,
    hash: &QrHash,
) -> Result<(), QrAdminDbError> {
    let exists: Option<i32> =
        sqlx::query_scalar("SELECT 1 FROM qr_codes WHERE code_hash = $1 FOR UPDATE")
            .bind(hash.as_bytes().as_slice())
            .fetch_optional(&mut **tx)
            .await?;
    if exists.is_some() {
        Ok(())
    } else {
        Err(QrAdminDbError::NotFound)
    }
}

async fn current_mapping_state_tx(
    tx: &mut Transaction<'_, Postgres>,
    hash: &QrHash,
) -> Result<Option<CurrentMappingState>, QrAdminDbError> {
    sqlx::query_as(
        "SELECT target_url, is_disabled
         FROM qr_mapping_events
         WHERE qr_code_hash = $1
         ORDER BY id DESC
         LIMIT 1",
    )
    .bind(hash.as_bytes().as_slice())
    .fetch_optional(&mut **tx)
    .await
    .map_err(QrAdminDbError::from)
}

async fn insert_mapping_event_tx(
    tx: &mut Transaction<'_, Postgres>,
    hash: &QrHash,
    target_url: &str,
    is_disabled: bool,
    changed_by: UserId,
) -> Result<MappingEvent, sqlx::Error> {
    sqlx::query_as::<_, MappingEvent>(
        "INSERT INTO qr_mapping_events (qr_code_hash, target_url, is_disabled, changed_by)
         VALUES ($1, $2, $3, $4)
         RETURNING id, target_url, is_disabled, changed_at, changed_by",
    )
    .bind(hash.as_bytes().as_slice())
    .bind(target_url)
    .bind(is_disabled)
    .bind(changed_by.num())
    .fetch_one(&mut **tx)
    .await
}

pub(crate) async fn current_mapping(
    db: &PgPool,
    hash: &QrHash,
) -> Result<Option<CurrentMapping>, sqlx::Error> {
    sqlx::query_as::<_, CurrentMapping>(
        "SELECT id AS event_id, target_url, is_disabled
         FROM qr_mapping_events
         WHERE qr_code_hash = $1
         ORDER BY id DESC
         LIMIT 1",
    )
    .bind(hash.as_bytes().as_slice())
    .fetch_optional(db)
    .await
}

pub(crate) async fn list_codes(
    db: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<QrCodeSummary>, sqlx::Error> {
    sqlx::query_as::<_, QrCodeSummary>(
        "SELECT encode(c.code_hash, 'hex') AS code_hash, c.code, c.created_at, c.created_by,
                e.id AS current_event_id, e.target_url AS current_target_url,
                e.is_disabled AS current_is_disabled, e.changed_at AS current_changed_at
         FROM qr_codes c
         LEFT JOIN LATERAL (
             SELECT id, target_url, is_disabled, changed_at
             FROM qr_mapping_events
             WHERE qr_code_hash = c.code_hash
             ORDER BY id DESC
             LIMIT 1
         ) e ON TRUE
         ORDER BY c.created_at DESC, c.code_hash
         LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
}

pub(crate) async fn code_detail(
    db: &PgPool,
    hash: &QrHash,
) -> Result<Option<QrCodeDetail>, sqlx::Error> {
    sqlx::query_as::<_, QrCodeDetail>(
        "SELECT encode(code_hash, 'hex') AS code_hash, code, created_at, created_by
         FROM qr_codes WHERE code_hash = $1",
    )
    .bind(hash.as_bytes().as_slice())
    .fetch_optional(db)
    .await
}

pub(crate) async fn mapping_history(
    db: &PgPool,
    hash: &QrHash,
) -> Result<Vec<MappingHistoryEvent>, sqlx::Error> {
    sqlx::query_as::<_, MappingHistoryEvent>(
        "SELECT id, target_url, is_disabled, changed_at,
                lead(changed_at) OVER (ORDER BY id) AS active_until,
                changed_by
         FROM qr_mapping_events
         WHERE qr_code_hash = $1
         ORDER BY id ASC",
    )
    .bind(hash.as_bytes().as_slice())
    .fetch_all(db)
    .await
}

pub(crate) async fn access_buckets(
    db: &PgPool,
    hash: &QrHash,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<AccessMinuteBucket>, sqlx::Error> {
    sqlx::query_as::<_, AccessMinuteBucket>(
        "SELECT b.mapping_event_id, e.target_url, b.bucket_start, b.access_count
         FROM qr_access_minute_buckets b
         JOIN qr_mapping_events e ON e.id = b.mapping_event_id
         WHERE e.qr_code_hash = $1 AND b.bucket_start >= $2 AND b.bucket_start < $3
         ORDER BY b.bucket_start ASC, b.mapping_event_id ASC",
    )
    .bind(hash.as_bytes().as_slice())
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await
}

/// Increment the current mapping version's UTC minute bucket.
pub(crate) async fn record_access(db: &PgPool, mapping_event_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO qr_access_minute_buckets (mapping_event_id, bucket_start, access_count)
         VALUES ($1, date_trunc('minute', clock_timestamp()), 1)
         ON CONFLICT (mapping_event_id, bucket_start)
         DO UPDATE SET access_count = qr_access_minute_buckets.access_count + 1",
    )
    .bind(mapping_event_id)
    .execute(db)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_ascii_url_safe_and_hash_exact_bytes() {
        let code = QrCode::parse("000AbC-_".to_owned()).unwrap();
        assert_eq!(code.as_str(), "000AbC-_");
        assert!(is_valid_qr_code(&"a".repeat(1024)));
        assert!(!is_valid_qr_code(""));
        assert!(!is_valid_qr_code(&"a".repeat(1025)));
        assert!(QrCode::parse("a/b".to_owned()).is_err());
        assert_eq!(
            QrCode::parse("abc".to_owned())
                .unwrap()
                .hash()
                .to_lower_hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn targets_require_a_trusted_non_qr_origin() {
        let app = Url::parse("https://app.bouncy-feet.ch/").unwrap();
        assert!(TargetUrl::parse(
            "https://app.bouncy-feet.ch/challenges/jumping-jacks/#details".to_owned(),
            &app,
            &[],
            false,
        )
        .is_ok());
        assert_eq!(
            TargetUrl::parse(
                "https://app.bouncy-feet.ch/qr/code/".to_owned(),
                &app,
                &[],
                false
            ),
            Err(TargetUrlError::QrLoop)
        );
    }

    #[sqlx::test(migrations = "./db_migrations")]
    async fn mappings_are_atomic_append_only_and_serialized(pool: PgPool) -> sqlx::Result<()> {
        let actor: i64 =
            sqlx::query_scalar("INSERT INTO users (oidc_subject) VALUES (NULL) RETURNING id")
                .fetch_one(&pool)
                .await?;
        let actor = UserId::from_i64(actor);
        let code = QrCode::parse("admin-code".to_owned()).unwrap();
        let hash = code.hash();
        let app = Url::parse("https://app.bouncy-feet.ch/").unwrap();
        let first = TargetUrl::parse(
            "https://app.bouncy-feet.ch/one/".to_owned(),
            &app,
            &[],
            false,
        )
        .unwrap();
        let second = TargetUrl::parse(
            "https://app.bouncy-feet.ch/two/".to_owned(),
            &app,
            &[],
            false,
        )
        .unwrap();

        let created = create_code_and_mapping(&pool, &code, &first, actor)
            .await
            .unwrap();
        assert_eq!(created.target_url, first.as_str());
        assert!(!created.is_disabled);
        assert!(matches!(
            create_code_and_mapping(&pool, &code, &first, actor).await,
            Err(QrAdminDbError::AlreadyConfigured)
        ));
        let no_change = append_mapping(&pool, &hash, &first, actor).await;
        assert!(
            matches!(no_change, Err(QrAdminDbError::NoChange)),
            "expected no-op conflict, got {no_change:?}"
        );
        append_mapping(&pool, &hash, &second, actor).await.unwrap();
        let disabled = disable_mapping(&pool, &hash, actor).await.unwrap();
        assert_eq!(disabled.target_url, second.as_str());
        assert!(disabled.is_disabled);
        append_mapping(&pool, &hash, &first, actor).await.unwrap();

        let history = mapping_history(&pool, &hash).await?;
        assert_eq!(history.len(), 4);
        assert!(history
            .iter()
            .take(3)
            .all(|event| event.active_until.is_some()));
        assert!(history.last().unwrap().active_until.is_none());
        assert!(sqlx::query("DELETE FROM qr_mapping_events WHERE id = $1")
            .bind(created.id)
            .execute(&pool)
            .await
            .is_err());
        Ok(())
    }
}
