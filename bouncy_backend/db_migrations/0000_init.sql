-- Lists all users.
CREATE TABLE IF NOT EXISTS users (
    -- internal DB id
    id BIGSERIAL PRIMARY KEY,
    -- Keycloak ID, or null for guests
    -- note: pg checks treat null values as distinct
    oidc_subject TEXT UNIQUE
);
-- Meta data about a user, in a flexible key-value store.
--
-- The sole source-of-truth for this data is the API server.
CREATE TABLE IF NOT EXISTS user_meta (
    user_id BIGINT REFERENCES users(id) NOT NULL,
    key_name VARCHAR(64) NOT NULL,
    key_value VARCHAR(1024) NOT NULL,
    -- Always stored as UTC
    last_modified TIMESTAMP,
    version_nr SMALLINT NOT NULL,
    PRIMARY KEY(user_id, key_name)
);
-- Client sessions contain per-device information of a user, even if they are
-- not registered, yet. Multiple sessions can exists per user. Users can claim
-- unowned sessions and merge them in to their profile, if they have access to
-- the session.
-- A session can be claimed either by either the client_session_secret xor by
-- the user_id, depending on which is set.
CREATE TABLE IF NOT EXISTS client_session (
    id BIGSERIAL PRIMARY KEY,
    -- client_session_secret is optional, if it is null, this is a registered user's session
    client_session_secret UUID,
    -- connected user can be a guest or a fully registered user
    user_id BIGINT REFERENCES users(id) NOT NULL,
    -- Always stored as UTC
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
-- Stores all recorded activities of all users ever.
--
-- The source-of-truth for this data is on the clients. But the data is
-- immutable. So any data the API server has is final. It might just not know
-- all the data, yet.
CREATE TABLE IF NOT EXISTS dance_activity (
    dance_activity_id BIGSERIAL PRIMARY KEY,
    client_session_id BIGINT REFERENCES client_session(id),
    -- Always stored as UTC
    recorded_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- identifier that encodes for what was danced, could be a specific lesson, or a step with a certain speed
    activity_id VARCHAR(256) NOT NULL,
    -- how many poses were hit
    hits INTEGER NOT NULL CHECK (hits >= 0),
    -- how many poses were missed
    misses INTEGER NOT NULL CHECK (misses >= 0),
    -- how many steps there were in total
    steps INTEGER NOT NULL CHECK (steps >= 0)
);
CREATE INDEX idx_dance_activity_client_session ON dance_activity(client_session_id);