-- Add up migration script here
CREATE TABLE training_results (
	id BIGSERIAL PRIMARY KEY NOT NULL,
	user_id BIGSERIAL NOT NULL,
	training_id BIGSERIAL NOT NULL,
	weight_value FLOAT NOT NULL,
	count_value INTEGER NOT NULL,
	done_at Timestamp NOT NULL
);
