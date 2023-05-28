-- Add up migration script here
CREATE TABLE trainings (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(255),
	default_weight_value FLOAT NOT NULL,
	default_count_value INTEGER NOT NULL
);
