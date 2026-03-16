ALTER TABLE video_beats
    ADD COLUMN pose_file TEXT; -- gzip compressed, then base64 encoded PoseFile
