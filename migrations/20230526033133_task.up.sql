-- Add up migration script here
CREATE TABLE task (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  usr_id BIGSERIAL NOT NULL,
  name VARCHAR(32) NOT NULL,
  description VARCHAR(256),
  FOREIGN KEY (usr_id) REFERENCES usr(id) ON DELETE CASCADE
);
