CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    oidc_subject TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS user_meta (
    user_id BIGSERIAL REFERENCES users(id),
    key_name VARCHAR(64) NOT NULL,
    key_value VARCHAR(1024) NOT NULL,
    last_modified TIMESTAMP,
    version_nr SMALLINT NOT NULL,
    PRIMARY KEY(user_id, key_name)
);
