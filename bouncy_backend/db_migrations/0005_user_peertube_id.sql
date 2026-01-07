ALTER TABLE users
-- account id for the user itself
-- NOT for a channel
-- NOT the user id
ADD COLUMN peertube_account_id BIGINT
;