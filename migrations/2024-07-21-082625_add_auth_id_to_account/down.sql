-- This file should undo anything in `up.sql`
ALTER TABLE account
DROP CONSTRAINT fk_auth;

ALTER TABLE account
DROP COLUMN auth_id;

