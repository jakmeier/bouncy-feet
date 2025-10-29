CREATE TABLE IF NOT EXISTS http_sessions
(
    -- i128 converted to uuid
    id UUID PRIMARY KEY,
    -- Always stored as UTC
    expiry_date TIMESTAMP NOT NULL,
    -- Raw session data, in Rust stored as HashMap<String, serde_json::Value>
    session_data JSONB
);