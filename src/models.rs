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

pub mod extract {
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
        pub count: i64,
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
        pub active_trainings: Vec<ActiveTraining>,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct ActiveTraining {
        pub training_id: i64,
        pub weight: f64,
        pub count: i32,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct DeleteTask {
        pub user_token: String,
        pub id: i64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct GetTrainings {
        pub count: i64,
        pub offset: i64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct GetTraining {
        pub id: i64,
    }
}

pub mod reply {
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
}
