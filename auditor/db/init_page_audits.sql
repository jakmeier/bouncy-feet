-- One row per audit run (a full sweep of all URLs)
CREATE TABLE IF NOT EXISTS audit_runs (
  id          SERIAL PRIMARY KEY,
  run_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
  environment TEXT NOT NULL,          -- 'staging', 'production', or 'local'
  git_commit  TEXT,                   -- optional
  notes       TEXT
);

-- One row per URL per run
CREATE TABLE IF NOT EXISTS page_audits (
  id              SERIAL PRIMARY KEY,
  run_id          INTEGER NOT NULL REFERENCES audit_runs(id) ON DELETE CASCADE,
  url             TEXT NOT NULL,
  load_time_ms    INTEGER,
  ttfb_ms         NUMERIC(8,2),
  dns_ms          NUMERIC(8,2),
  tcp_ms          NUMERIC(8,2),
  dom_ready_ms    NUMERIC(8,2),
  full_load_ms    NUMERIC(8,2),
  lcp_ms          NUMERIC(8,2),
  total_requests  INTEGER,
  total_kb        NUMERIC(10,2),
  recorded_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- One row per resource type per page audit (script/fetch/img/doc etc.)
CREATE TABLE IF NOT EXISTS request_breakdown (
  id            SERIAL PRIMARY KEY,
  page_audit_id INTEGER NOT NULL REFERENCES page_audits(id) ON DELETE CASCADE,
  resource_type TEXT NOT NULL,    -- 'script', 'fetch', 'image', 'document'...
  request_count INTEGER NOT NULL
);

CREATE INDEX ON page_audits (run_id);
CREATE INDEX ON page_audits (url, recorded_at);
CREATE INDEX ON request_breakdown (page_audit_id);
