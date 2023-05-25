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
    pub user_id: i64,
    pub training_id: i64,
    pub weight: f32,
    pub count: i32,
    pub at: NaiveDateTime,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActiveTraining {
    pub task_id: i64,
    pub order: i64,
    pub weight: f32,
    pub count: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Training {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub default_weight: f32,
    pub default_count: i32,
}

pub mod extract {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct GetUser {
        pub token: String,
    }
}

pub mod reply {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Meta {
        pub name: String,
        pub version: String,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct GetUser {
        pub token: String,
    }
}
