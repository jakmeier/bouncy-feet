ALTER TABLE clubs
ADD COLUMN channel_id BIGINT;

CREATE TABLE club_playlists (
    id BIGSERIAL PRIMARY KEY,

    -- owning club
    club_id BIGINT NOT NULL
        REFERENCES clubs(id)
        ON DELETE CASCADE,

    -- external playlist data
    playlist_id BIGINT NOT NULL,
    playlist_short_uuid VARCHAR(32) NOT NULL,

    -- visibility
    is_private BOOLEAN NOT NULL,

    -- a club should not accidentally have the same playlist twice
    UNIQUE (club_id, playlist_id)
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
    REFERENCES club_playlists(id);

-- populate main_playlist
UPDATE clubs c
SET main_playlist = cp.id
FROM club_playlists cp
WHERE cp.club_id = c.id
  AND cp.is_private = FALSE;

-- now remove old playlist fields on club
ALTER TABLE clubs
DROP COLUMN public_playlist_id,
DROP COLUMN public_playlist_short_uuid,
DROP COLUMN private_playlist_id,
DROP COLUMN private_playlist_short_uuid;

-- add constraints
ALTER TABLE club_playlists
ADD CONSTRAINT club_playlists_id_club_unique
UNIQUE (id, club_id);

-- add indexes
CREATE INDEX idx_club_playlists_club_id
ON club_playlists(club_id);