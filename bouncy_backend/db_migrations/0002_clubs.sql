CREATE TABLE IF NOT EXISTS clubs (
    -- internal DB id
    id BIGSERIAL PRIMARY KEY,
    -- displayed title (language agnostic)
    title VARCHAR(64) NOT NULL,
    -- displayed description (language agnostic)
    description VARCHAR(1024) NOT NULL,
    -- peertube playlist id for videos shown on the public profile of this club
    public_playlist VARCHAR(32) NOT NULL,
    -- peertube playlist id for videos only visible by members
    private_playlist VARCHAR(32) NOT NULL
);

-- club membership
CREATE TABLE IF NOT EXISTS user_club (
    user_id BIGINT REFERENCES users(id) NOT NULL,
    club_id BIGINT REFERENCES clubs(id) NOT NULL,
    is_admin BOOLEAN NOT NULL,
    PRIMARY KEY(user_id, club_id)
);
