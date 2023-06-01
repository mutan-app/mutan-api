-- Add up migration script here
CREATE TABLE train_res (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  usr_id BIGSERIAL NOT NULL,
  train_id BIGSERIAL NOT NULL,
  weight_val FLOAT NOT NULL,
  count_val INTEGER NOT NULL,
  done_at Timestamp NOT NULL,
  FOREIGN KEY (usr_id) REFERENCES usr(id) ON DELETE CASCADE,
  FOREIGN kEY (train_id) REFERENCES task(id) ON DELETE CASCADE
);
