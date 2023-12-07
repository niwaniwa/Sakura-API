-- Your SQL goes here
CREATE TABLE account (
  id bigserial NOT NULL PRIMARY KEY,
  username varchar(255) NOT NULL UNIQUE,
  grade INTEGER CHECK (grade >= 1 AND grade <= 4) NOT NULL,
  card_type varchar(255) NOT NULL,
  card_id bytea NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
