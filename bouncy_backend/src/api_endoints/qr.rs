use crate::db::qr::{
    access_buckets, append_mapping, code_detail, create_code_and_mapping, current_mapping,
    disable_mapping, list_codes, mapping_history, record_access, AccessMinuteBucket, MappingEvent,
    MappingHistoryEvent, QrAdminDbError, QrCode, QrHash, TargetUrl,
};
use crate::user::User;
use crate::{db_err_to_response, AppState};
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::{Extension, Json};
use chrono::{DateTime, Duration, Utc};

const NO_STORE: HeaderValue = HeaderValue::from_static("no-store");
const MAX_LIST_LIMIT: i64 = 100;
const DEFAULT_LIST_LIMIT: i64 = 50;
const MAX_STATS_RANGE: Duration = Duration::days(31);

#[derive(serde::Serialize)]
struct ResolveResponse {
    target_url: String,
}

/// Resolve an opaque QR digest to its currently active target.
///
/// The browser, rather than this endpoint, performs the final navigation. This
/// keeps failures on the branded frontend page and avoids fetch redirect traps.
pub async fn resolve(State(state): State<AppState>, Path(code_hash): Path<String>) -> Response {
    let hash = match QrHash::parse_lower_hex(&code_hash) {
        Ok(hash) => hash,
        Err(_) => return no_store_status(StatusCode::BAD_REQUEST),
    };
    let hash_text = hash.to_lower_hex();

    let mapping = match current_mapping(&state.pg_db_pool, &hash).await {
        Ok(Some(mapping)) if !mapping.is_disabled => mapping,
        Ok(_) => return no_store_status(StatusCode::NOT_FOUND),
        Err(error) => {
            tracing::error!(qr_hash = %hash_text, ?error, "QR mapping lookup failed");
            return no_store_status(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let target_url = mapping.target_url;
    tracing::info!(
        qr_hash = %hash_text,
        mapping_event_id = mapping.event_id,
        "QR mapping resolved"
    );

    // Stats are deliberately best-effort and detached from the response path:
    // a healthy mapping must forward even while its bucket is being written.
    let db = state.pg_db_pool.clone();
    let mapping_event_id = mapping.event_id;
    tokio::spawn(async move {
        if let Err(error) = record_access(&db, mapping_event_id).await {
            tracing::error!(
                qr_hash = %hash_text,
                mapping_event_id,
                ?error,
                "QR access bucket update failed"
            );
        }
    });

    (
        StatusCode::OK,
        [(header::CACHE_CONTROL, NO_STORE)],
        Json(ResolveResponse { target_url }),
    )
        .into_response()
}

#[derive(serde::Deserialize)]
pub(crate) struct CreateQrRequest {
    code: String,
    target_url: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct ChangeMappingRequest {
    target_url: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct ListQrParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(serde::Deserialize)]
pub(crate) struct StatsParams {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

#[derive(serde::Serialize)]
struct ListQrResponse<T> {
    mappings: Vec<T>,
}

#[derive(serde::Serialize)]
struct CreatedQrResponse {
    code_hash: String,
    event: MappingEvent,
}

#[derive(serde::Serialize)]
struct QrDetailResponse {
    code_hash: String,
    code: String,
    created_at: DateTime<Utc>,
    created_by: i64,
    history: Vec<MappingHistoryEvent>,
}

#[derive(serde::Serialize)]
struct QrStatsResponse {
    buckets: Vec<AccessMinuteBucket>,
}

pub(crate) async fn admin_ui() -> Response {
    ([(header::CACHE_CONTROL, NO_STORE)], Html(ADMIN_QR_HTML)).into_response()
}

/// List current QR configurations, including disabled ones. This response is
/// role-protected and intentionally contains the stored printed code.
pub(crate) async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListQrParams>,
) -> Response {
    let limit = params.limit.unwrap_or(DEFAULT_LIST_LIMIT);
    let offset = params.offset.unwrap_or(0);
    if !(1..=MAX_LIST_LIMIT).contains(&limit) || offset < 0 {
        return bad_request("limit must be 1 to 100 and offset must not be negative");
    }

    match list_codes(&state.pg_db_pool, limit, offset).await {
        Ok(mappings) => Json(ListQrResponse { mappings }).into_response(),
        Err(error) => db_err_to_response(error),
    }
}

/// Create a code and its first active mapping in one append-only transaction.
pub(crate) async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(request): Json<CreateQrRequest>,
) -> Response {
    let code = match QrCode::parse(request.code) {
        Ok(code) => code,
        Err(error) => return bad_request(error.to_string()),
    };
    let hash = code.hash();
    let target = match validated_target(&state, request.target_url) {
        Ok(target) => target,
        Err(message) => return bad_request(message),
    };
    match create_code_and_mapping(&state.pg_db_pool, &code, &target, user.id).await {
        Ok(event) => (
            StatusCode::CREATED,
            Json(CreatedQrResponse {
                code_hash: hash.to_lower_hex(),
                event,
            }),
        )
            .into_response(),
        Err(error) => admin_db_error(&hash, error),
    }
}

pub(crate) async fn detail(
    State(state): State<AppState>,
    Path(code_hash): Path<String>,
) -> Response {
    let hash = match parse_admin_hash(&code_hash) {
        Ok(hash) => hash,
        Err(message) => return bad_request(message),
    };
    let detail = match code_detail(&state.pg_db_pool, &hash).await {
        Ok(Some(detail)) => detail,
        Ok(None) => return (StatusCode::NOT_FOUND, "QR code not found").into_response(),
        Err(error) => return db_err_to_response(error),
    };
    match mapping_history(&state.pg_db_pool, &hash).await {
        Ok(history) => Json(QrDetailResponse {
            code_hash: detail.code_hash,
            code: detail.code,
            created_at: detail.created_at,
            created_by: detail.created_by,
            history,
        })
        .into_response(),
        Err(error) => db_err_to_response(error),
    }
}

pub(crate) async fn change_mapping(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(code_hash): Path<String>,
    Json(request): Json<ChangeMappingRequest>,
) -> Response {
    let hash = match parse_admin_hash(&code_hash) {
        Ok(hash) => hash,
        Err(message) => return bad_request(message),
    };
    let target = match validated_target(&state, request.target_url) {
        Ok(target) => target,
        Err(message) => return bad_request(message),
    };
    match append_mapping(&state.pg_db_pool, &hash, &target, user.id).await {
        Ok(event) => (StatusCode::CREATED, Json(event)).into_response(),
        Err(error) => admin_db_error(&hash, error),
    }
}

pub(crate) async fn disable(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(code_hash): Path<String>,
) -> Response {
    let hash = match parse_admin_hash(&code_hash) {
        Ok(hash) => hash,
        Err(message) => return bad_request(message),
    };
    match disable_mapping(&state.pg_db_pool, &hash, user.id).await {
        Ok(event) => (StatusCode::CREATED, Json(event)).into_response(),
        Err(error) => admin_db_error(&hash, error),
    }
}

pub(crate) async fn stats(
    State(state): State<AppState>,
    Path(code_hash): Path<String>,
    Query(params): Query<StatsParams>,
) -> Response {
    let hash = match parse_admin_hash(&code_hash) {
        Ok(hash) => hash,
        Err(message) => return bad_request(message),
    };
    if params.from >= params.to || params.to - params.from > MAX_STATS_RANGE {
        return bad_request("from and to must define a range of at most 31 days");
    }
    match code_detail(&state.pg_db_pool, &hash).await {
        Ok(Some(_)) => {}
        Ok(None) => return (StatusCode::NOT_FOUND, "QR code not found").into_response(),
        Err(error) => return db_err_to_response(error),
    }
    match access_buckets(&state.pg_db_pool, &hash, params.from, params.to).await {
        Ok(buckets) => Json(QrStatsResponse { buckets }).into_response(),
        Err(error) => db_err_to_response(error),
    }
}

fn validated_target(state: &AppState, target_url: String) -> Result<TargetUrl, String> {
    TargetUrl::parse(
        target_url,
        &state.app_url,
        &state.qr_trusted_target_origins,
        state.qr_allow_localhost_http,
    )
    .map_err(|error| error.to_string())
}

fn parse_admin_hash(value: &str) -> Result<QrHash, &'static str> {
    QrHash::parse_lower_hex(value).map_err(|_| "invalid QR hash")
}

fn admin_db_error(hash: &QrHash, error: QrAdminDbError) -> Response {
    match error {
        QrAdminDbError::AlreadyConfigured | QrAdminDbError::NoChange => {
            (StatusCode::CONFLICT, error.to_string()).into_response()
        }
        QrAdminDbError::NotFound => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
        QrAdminDbError::HashCollision => {
            tracing::error!(qr_hash = %hash.to_lower_hex(), "QR hash collision detected");
            (StatusCode::CONFLICT, "QR hash collision").into_response()
        }
        QrAdminDbError::Database(error) => db_err_to_response(error),
    }
}

fn bad_request(message: impl Into<String>) -> Response {
    (StatusCode::BAD_REQUEST, message.into()).into_response()
}

fn no_store_status(status: StatusCode) -> Response {
    (status, [(header::CACHE_CONTROL, NO_STORE)]).into_response()
}

const ADMIN_QR_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>QR administration</title>
</head>
<body>
  <main>
    <h1>QR administration</h1>
    <p>This page is available only to users with the <code>qr-admin</code> role.</p>

    <h2>Create QR mapping</h2>
    <form id="create-form">
      <p><label>Printed code <input name="code" required pattern="[A-Za-z0-9_-]{1,1024}"></label></p>
      <p><label>Target URL <input name="target_url" type="url" required maxlength="2048"></label></p>
      <p><button>Create mapping</button></p>
    </form>

    <h2>Retarget QR mapping</h2>
    <form id="mapping-form">
      <p><label>QR hash <input name="hash" required pattern="[0-9a-f]{64}"></label></p>
      <p><label>Target URL <input name="target_url" type="url" required maxlength="2048"></label></p>
      <p><button>Append mapping</button></p>
    </form>

    <h2>Disable QR mapping</h2>
    <form id="disable-form">
      <p><label>QR hash <input name="hash" required pattern="[0-9a-f]{64}"></label></p>
      <p><button>Disable mapping</button></p>
    </form>

    <p id="status" role="status"></p>
    <h2>Configured QR mappings</h2>
    <button id="refresh" type="button">Refresh list</button>
    <table>
      <thead><tr><th>Code</th><th>Hash</th><th>Current target</th><th>Status</th></tr></thead>
      <tbody id="mappings"></tbody>
    </table>
  </main>

  <script>
    const status = document.querySelector('#status');
    const mappings = document.querySelector('#mappings');

    function showStatus(message) {
      status.textContent = message;
    }

    async function request(path, options) {
      const response = await fetch(path, options);
      if (!response.ok) {
        throw new Error(await response.text() || `Request failed (${response.status})`);
      }
      return response.status === 204 ? null : response.json();
    }

    async function refresh() {
      try {
        const response = await request('/admin/qr?limit=100');
        mappings.replaceChildren();
        for (const mapping of response.mappings) {
          const row = document.createElement('tr');
          const cells = [
            mapping.code,
            mapping.code_hash,
            mapping.current_target_url || '',
            mapping.current_is_disabled ? 'disabled' : 'active',
          ];
          for (const value of cells) {
            const cell = document.createElement('td');
            cell.textContent = value;
            row.append(cell);
          }
          mappings.append(row);
        }
        showStatus(`Loaded ${response.mappings.length} mapping(s).`);
      } catch (error) {
        showStatus(error.message);
      }
    }

    function formData(form) {
      return Object.fromEntries(new FormData(form));
    }

    document.querySelector('#create-form').addEventListener('submit', async (event) => {
      event.preventDefault();
      try {
        const result = await request('/admin/qr', {
          method: 'POST',
          headers: { 'content-type': 'application/json' },
          body: JSON.stringify(formData(event.currentTarget)),
        });
        showStatus(`Created mapping ${result.code_hash}.`);
        event.currentTarget.reset();
        await refresh();
      } catch (error) {
        showStatus(error.message);
      }
    });

    document.querySelector('#mapping-form').addEventListener('submit', async (event) => {
      event.preventDefault();
      const values = formData(event.currentTarget);
      const hash = values.hash;
      delete values.hash;
      try {
        await request(`/admin/qr/${hash}/mapping`, {
          method: 'POST',
          headers: { 'content-type': 'application/json' },
          body: JSON.stringify(values),
        });
        showStatus(`Updated mapping ${hash}.`);
        await refresh();
      } catch (error) {
        showStatus(error.message);
      }
    });

    document.querySelector('#disable-form').addEventListener('submit', async (event) => {
      event.preventDefault();
      const hash = formData(event.currentTarget).hash;
      try {
        await request(`/admin/qr/${hash}/disable`, { method: 'POST' });
        showStatus(`Disabled mapping ${hash}.`);
        await refresh();
      } catch (error) {
        showStatus(error.message);
      }
    });

    document.querySelector('#refresh').addEventListener('click', refresh);
    refresh();
  </script>
</body>
</html>"#;
