-- Your SQL goes here
ALTER TABLE account
ADD COLUMN auth_id BIGINT NOT NULL;

ALTER TABLE account
ADD CONSTRAINT fk_auth
FOREIGN KEY (auth_id) REFERENCES auth(id);

