-- Add up migration script here
CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  token VARCHAR(255) UNIQUE NOT NULL
);
