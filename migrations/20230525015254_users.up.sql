-- Add up migration script here
CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  token VARCHAR(256) UNIQUE NOT NULL
);
