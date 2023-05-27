-- Add up migration script here
CREATE TABLE active_trainings (
  id BIGSERIAL PRIMARY KEY NOT NULL,
	task_id BIGSERIAL NOT NULL,
	training_id BIGSERIAL NOT NULL,
	target_order INTEGER NOT NULL,
	target_weight FLOAT NOT NULL,
	target_count INTEGER NOT NULL,
	FOREIGN KEY (task_id) REFERENCES tasks(id),
	FOREIGN KEY (training_id) REFERENCES trainings(id)
);
