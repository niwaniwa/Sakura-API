-- Your SQL goes here
CREATE TABLE auth (
  id bigserial NOT NULL PRIMARY KEY,
  email varchar(100) NOT NULL UNIQUE,
  password varchar(255) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
