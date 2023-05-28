use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Meta {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetUser {
    pub token: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct CreateUser {
    pub token: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetTasks {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetTask {
    pub task: Task,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Task {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetTrainings {
    pub trainings: Vec<Training>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetTraining {
    pub training: Training,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Training {
    pub name: String,
    pub description: Option<String>,
    pub default_weight_value: f64,
    pub default_count_value: i32,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetTaskInstances {
    pub task_instances: Vec<TaskInstance>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetTaskInstance {
    pub task_instance: TaskInstance,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct TaskInstance {
    pub task_id: i64,
    pub progress_value: i32,
}
