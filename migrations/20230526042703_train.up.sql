-- Add up migration script here
CREATE TABLE train (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(32) NOT NULL,
  description VARCHAR(256),
  weight FLOAT NOT NULL,
  times INTEGER NOT NULL,
  tags VARCHAR(32) ARRAY
);
