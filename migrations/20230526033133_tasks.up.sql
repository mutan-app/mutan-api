-- Add up migration script here
CREATE TABLE tasks (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  user_id BIGSERIAL NOT NULL,
  name VARCHAR(32) NOT NULL,
  description VARCHAR(256),
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
