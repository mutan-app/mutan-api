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
pub struct DoneTask {
    pub id: i64,
    pub task_id: i64,
    pub at: NaiveDateTime,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActiveTask {
    pub id: i64,
    pub user_id: i64,
    pub task_id: i64,
    pub progress: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DoneTraining {
    pub id: i64,
    pub user_id: i64,
    pub training_id: i64,
    pub weight: f64,
    pub count: i32,
    pub at: NaiveDateTime,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActiveTraining {
    pub task_id: i64,
    pub training_id: i64,
    pub order: i32,
    pub weight: f64,
    pub count: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Training {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub default_weight: f64,
    pub default_count: i32,
}
