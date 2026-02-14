ALTER TABLE users
-- account handle for the user itself
-- NOT for a channel of the user but the user itself
ADD COLUMN peertube_handle VARCHAR(255)
;
