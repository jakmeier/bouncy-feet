-- ALTER TABLE page_audits ADD COLUMN lcp_ms NUMERIC(8,2);

ALTER TABLE page_audits ADD COLUMN precached BOOLEAN NOT NULL DEFAULT false;