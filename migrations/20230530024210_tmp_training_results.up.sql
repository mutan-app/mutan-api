-- Add up migration script here
CREATE TABLE tmp_training_results (
	id BIGSERIAL PRIMARY KEY NOT NULL,
	task_instance_id BIGSERIAL NOT NULL,
	training_instance_id BIGSERIAL NOT NULL,
	done_at Timestamp NOT NULL,
	FOREIGN KEY (training_instance_id) REFERENCES training_instances(id) ON DELETE CASCADE
);
