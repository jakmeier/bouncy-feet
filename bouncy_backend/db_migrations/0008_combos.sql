-- Step combinations, uploadable by users.
CREATE TABLE IF NOT EXISTS combos (
    -- internal DB id
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) NOT NULL,

    -- unlisted or public on PeerTube
    is_private BOOLEAN NOT NULL,
    -- for display order
    sort_order INT,
    -- for group-by display on a user profile
    free_form_category VARCHAR(64),

    -- Below information is optional. On creation, only a video is necessary but
    -- even that can be deleted later.
    -- displayed title (language agnostic)
    title VARCHAR(64),
    video_short_uuid VARCHAR(22)
);
