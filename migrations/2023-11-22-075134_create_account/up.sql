-- Your SQL goes here
CREATE TABLE account (
  id bigserial NOT NULL PRIMARY KEY,
  username varchar(255) NOT NULL UNIQUE,
  card_id bytea NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
