CREATE TABLE qr_codes (
    code_hash BYTEA PRIMARY KEY CHECK (octet_length(code_hash) = 32),
    code TEXT NOT NULL UNIQUE CHECK (
        octet_length(code) BETWEEN 1 AND 1024
        AND code ~ '^[A-Za-z0-9_-]+$'
    ),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    created_by BIGINT NOT NULL REFERENCES users(id)
);

CREATE TABLE qr_mapping_events (
    id BIGSERIAL PRIMARY KEY,
    qr_code_hash BYTEA NOT NULL REFERENCES qr_codes(code_hash),
    target_url VARCHAR(2048) NOT NULL,
    is_disabled BOOLEAN NOT NULL DEFAULT FALSE,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    changed_by BIGINT NOT NULL REFERENCES users(id)
);

CREATE INDEX qr_mapping_events_current_idx
    ON qr_mapping_events (qr_code_hash, id DESC);

CREATE TABLE qr_access_minute_buckets (
    mapping_event_id BIGINT NOT NULL REFERENCES qr_mapping_events(id),
    bucket_start TIMESTAMPTZ NOT NULL CHECK (
        bucket_start = date_trunc('minute', bucket_start)
    ),
    access_count BIGINT NOT NULL CHECK (access_count > 0),
    PRIMARY KEY (mapping_event_id, bucket_start)
);

CREATE FUNCTION reject_qr_audit_mutation()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    RAISE EXCEPTION '% is append-only', TG_TABLE_NAME;
END;
$$;

CREATE TRIGGER qr_codes_append_only
    BEFORE UPDATE OR DELETE ON qr_codes
    FOR EACH ROW EXECUTE FUNCTION reject_qr_audit_mutation();

CREATE TRIGGER qr_mapping_events_append_only
    BEFORE UPDATE OR DELETE ON qr_mapping_events
    FOR EACH ROW EXECUTE FUNCTION reject_qr_audit_mutation();
