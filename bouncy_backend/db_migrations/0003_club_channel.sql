ALTER TABLE clubs
ADD COLUMN channel_id BIGINT;

CREATE TABLE IF NOT EXISTS club_playlists (
    -- external playlist data
    -- playlist_id is also as primary key in this db
    playlist_id BIGINT PRIMARY KEY,
    playlist_short_uuid VARCHAR(32) NOT NULL,

    -- local data
    -- owning club
    club_id BIGINT NOT NULL
        REFERENCES clubs(id)
        ON DELETE CASCADE,
    is_private BOOLEAN NOT NULL
);

-- migrate existing public/private playlist pairs
-- previously stored directly on clubs, in two queries
INSERT INTO club_playlists (
    club_id,
    playlist_id,
    playlist_short_uuid,
    is_private
)
SELECT
    id,
    public_playlist_id,
    public_playlist_short_uuid,
    FALSE
FROM clubs;

INSERT INTO club_playlists (
    club_id,
    playlist_id,
    playlist_short_uuid,
    is_private
)
SELECT
    id,
    private_playlist_id,
    private_playlist_short_uuid,
    TRUE
FROM clubs;

-- from now on, point to club_playlists form clubs to select main playlist
ALTER TABLE clubs
ADD COLUMN main_playlist BIGINT
    REFERENCES club_playlists(playlist_id);

-- populate main_playlist
UPDATE clubs c
SET main_playlist = cp.playlist_id
FROM club_playlists cp
WHERE cp.club_id = c.id
  AND cp.is_private = FALSE;

-- now remove old playlist fields on club
ALTER TABLE clubs
DROP COLUMN public_playlist_id,
DROP COLUMN public_playlist_short_uuid,
DROP COLUMN private_playlist_id,
DROP COLUMN private_playlist_short_uuid;

-- add indices
  -- for lookup of playlists by a club
CREATE INDEX idx_club_playlists_club_id
ON club_playlists(club_id);