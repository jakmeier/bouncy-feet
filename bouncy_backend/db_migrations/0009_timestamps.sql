-- Video timestamps with extra info for combos, choreos, or videos in general
CREATE TABLE IF NOT EXISTS video_timestamps (
    id BIGSERIAL PRIMARY KEY,
    milliseconds INT NOT NULL
    -- start with only a timestamp
    -- maybe add more info later, such as "marker type", "extra info", "avatar" etc.
    -- that could be in this or in another table
);

CREATE TABLE IF NOT EXISTS combos_video_timestamps (
    combo_id BIGINT REFERENCES combos(id) NOT NULL,
    video_timestamp_id BIGINT REFERENCES video_timestamps(id) NOT NULL,
    PRIMARY KEY(combo_id, video_timestamp_id)
);
