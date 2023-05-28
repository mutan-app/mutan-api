-- Add up migration script here
CREATE TABLE tasks (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  user_id BIGSERIAL NOT NULL,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(255),
	FOREIGN KEY (user_id) REFERENCES users(id)
);
