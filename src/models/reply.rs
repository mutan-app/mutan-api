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
    pub default_weight: f64,
    pub default_count: i32,
}
