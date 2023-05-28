-- Add up migration script here
CREATE TABLE training_instances (
  id BIGSERIAL PRIMARY KEY NOT NULL,
	task_id BIGSERIAL NOT NULL,
	order_value INTEGER NOT NULL,
	training_id BIGSERIAL NOT NULL,
	weight_value FLOAT NOT NULL,
	count_value INTEGER NOT NULL,
	FOREIGN KEY (task_id) REFERENCES tasks(id),
	FOREIGN KEY (training_id) REFERENCES trainings(id)
);
