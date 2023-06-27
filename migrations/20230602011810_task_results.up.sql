-- Add up migration script here
CREATE TABLE task_results (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    task_id BIGSERIAL NOT NULL,
    done_at TIMESTAMP NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);
