-- Add up migration script here
CREATE TABLE train_ins (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  task_id BIGSERIAL NOT NULL,
  order_val INTEGER NOT NULL,
  train_id BIGSERIAL NOT NULL,
  weight_val FLOAT NOT NULL,
  count_val INTEGER NOT NULL,
  FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE CASCADE,
  FOREIGN KEY (train_id) REFERENCES train(id) ON DELETE CASCADE
);
