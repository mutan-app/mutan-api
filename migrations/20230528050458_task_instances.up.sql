-- Add up migration script here
CREATE TABLE task_instances (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  task_id BIGSERIAL NOT NULL,
	progress_value INTEGER NOT NULL,
	FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);
