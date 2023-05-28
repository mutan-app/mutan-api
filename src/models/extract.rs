use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct GetUser {
    pub token: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct DeleteUser {
    pub token: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct GetTasks {
    pub user_token: String,
    pub size: i64,
    pub offset: i64,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct GetTask {
    pub user_token: String,
    pub id: i64,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct CreateTask {
    pub user_token: String,
    pub name: String,
    pub description: Option<String>,
    pub training_instances: Vec<TrainingInstance>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TrainingInstance {
    pub training_id: i64,
    pub weight_value: f64,
    pub count_vale: i32,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct DeleteTask {
    pub user_token: String,
    pub id: i64,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct GetTrainings {
    pub size: i64,
    pub offset: i64,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct GetTraining {
    pub id: i64,
}
