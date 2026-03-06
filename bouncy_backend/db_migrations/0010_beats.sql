-- A range with a known beat within a video
CREATE TABLE IF NOT EXISTS video_beats (
    id BIGSERIAL PRIMARY KEY,
    start INT NOT NULL, -- ms
    duration INT NOT NULL, -- ms
    bpm REAL NOT NULL, -- music speed
    subbeat_per_move SMALLINT NOT NULL -- how many half-beats correspond to one dance move
);

CREATE TABLE IF NOT EXISTS combos_video_beats (
    combo_id BIGINT REFERENCES combos(id) NOT NULL,
    video_beat_id BIGINT REFERENCES video_beats(id) NOT NULL,
    PRIMARY KEY(combo_id, video_beat_id)
);
