-- Add up migration script here
CREATE TABLE task_res (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  task_id BIGSERIAL NOT NULL,
  done_at TIMESTAMP NOT NULL,
  FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE CASCADE
);
