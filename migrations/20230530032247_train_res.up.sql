-- Add up migration script here
CREATE TABLE train_res (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  usr_id BIGSERIAL NOT NULL,
  train_id BIGSERIAL NOT NULL,
  weight FLOAT NOT NULL,
  times INTEGER NOT NULL,
  done_at TIMESTAMP NOT NULL,
  FOREIGN KEY (usr_id) REFERENCES usr(id) ON DELETE CASCADE,
  FOREIGN kEY (train_id) REFERENCES task(id) ON DELETE CASCADE
);
