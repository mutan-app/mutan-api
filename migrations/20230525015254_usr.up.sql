-- Add up migration script here
CREATE TABLE usr (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  token VARCHAR(256) UNIQUE NOT NULL
);
