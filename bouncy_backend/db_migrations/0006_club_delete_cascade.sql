ALTER TABLE user_club
DROP CONSTRAINT user_club_user_id_fkey,
DROP CONSTRAINT user_club_club_id_fkey,
ADD CONSTRAINT user_club_user_id_fkey 
    FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON DELETE CASCADE,
ADD CONSTRAINT user_club_club_id_fkey
    FOREIGN KEY (club_id)
    REFERENCES clubs(id)
    ON DELETE CASCADE;