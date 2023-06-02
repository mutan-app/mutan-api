-- Add up migration script here
CREATE TABLE training_instances (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  task_id BIGSERIAL NOT NULL,
  stage INTEGER NOT NULL,
  training_id BIGSERIAL NOT NULL,
  weight FLOAT NOT NULL,
  times INTEGER NOT NULL,
  FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
  FOREIGN KEY (training_id) REFERENCES trainings(id) ON DELETE CASCADE
);
