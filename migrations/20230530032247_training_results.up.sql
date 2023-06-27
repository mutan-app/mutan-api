-- Add up migration script here
CREATE TABLE training_results (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    user_id BIGSERIAL NOT NULL,
    training_id BIGSERIAL NOT NULL,
    weight FLOAT NOT NULL,
    times INTEGER NOT NULL,
    done_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN kEY (training_id) REFERENCES trainings(id) ON DELETE CASCADE
);
