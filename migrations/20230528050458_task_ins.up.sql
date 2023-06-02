-- Add up migration script here
CREATE TABLE task_ins (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  task_id BIGSERIAL NOT NULL,
  progress INTEGER NOT NULL,
  FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE CASCADE
);
