pub mod extract;
pub mod reply;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub type Db = std::sync::Arc<tokio::sync::Mutex<sqlx::PgPool>>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub token: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaskInstance {
    pub id: i64,
    pub task_id: i64,
    pub progress_value: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub id: i64,
    pub task_id: i64,
    pub at: NaiveDateTime,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Training {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub default_weight_value: f64,
    pub default_count_value: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TrainingInstance {
    pub id: i64,
    pub task_id: i64,
    pub order_value: i32,
    pub training_id: i64,
    pub weight_value: f64,
    pub count_value: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub id: i64,
    pub user_id: i64,
    pub training_id: i64,
    pub weight_value: f64,
    pub count_value: i32,
    pub at: NaiveDateTime,
}
