CREATE TABLE IF NOT EXISTS clubs_combos (
    club_id BIGINT REFERENCES clubs(id) NOT NULL,
    combo_id BIGINT REFERENCES combos(id) NOT NULL,
    -- private means it is only visible to club members
    is_private BOOLEAN NOT NULL,
    PRIMARY KEY(club_id, combo_id)
);
