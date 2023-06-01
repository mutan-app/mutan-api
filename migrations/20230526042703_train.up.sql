-- Add up migration script here
CREATE TABLE train (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(32) NOT NULL,
  description VARCHAR(256),
  weight_val FLOAT NOT NULL,
  count_val INTEGER NOT NULL,
  tags VARCHAR(32) ARRAY
);
