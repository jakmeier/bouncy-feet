ALTER TABLE combos_video_beats
    DROP CONSTRAINT combos_video_beats_video_beat_id_fkey;

ALTER TABLE combos_video_beats
ADD CONSTRAINT combos_video_beats_video_beat_id_fkey
    FOREIGN KEY (video_beat_id)
    REFERENCES video_beats(id)
    ON DELETE CASCADE;

ALTER TABLE combos_video_timestamps
    DROP CONSTRAINT combos_video_timestamps_video_timestamp_id_fkey;

ALTER TABLE combos_video_timestamps
ADD CONSTRAINT combos_video_timestamps_video_timestamp_id_fkey
    FOREIGN KEY (video_timestamp_id)
    REFERENCES video_timestamps(id)
    ON DELETE CASCADE;