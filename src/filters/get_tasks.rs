use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
    pub offset: i64,
    pub limit: i64,
    pub order_by: String,
}

struct Task {
    id: i64,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub training_instances: Vec<TrainingInstance>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct TrainingInstance {
    pub id: i64,
    pub training_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub weight: f64,
    pub times: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<Vec<Reply>, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    let tasks = match extract.order_by.as_str() {
        "new" => sqlx::query_as!(
            Task,
            "SELECT id, name, description FROM tasks WHERE user_id = $1
                ORDER BY id DESC
                OFFSET $2 LIMIT $3",
            user.id,
            extract.offset,
            extract.limit,
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get tasks")),

        "times" => sqlx::query_as!(
            Task,
            "SELECT id, name, description FROM tasks WHERE user_id = $1
                ORDER BY (SELECT COUNT(id) FROM task_results WHERE task_id = id) DESC
                OFFSET $2 LIMIT $3",
            user.id,
            extract.offset,
            extract.limit,
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get tasks")),

        "recent" => sqlx::query_as!(
            Task,
            "SELECT id, name, description FROM tasks WHERE user_id = $1
                ORDER BY (SELECT MAX(done_at) FROM task_results WHERE task_id = id) DESC
                OFFSET $2 LIMIT $3",
            user.id,
            extract.offset,
            extract.limit,
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get tasks")),

        _ => Err(util::ErrorMessage::new("invalid order_by value")),
    }?;

    let mut reply_all = vec![];
    for task in tasks {
        let training_instances = sqlx::query_as!(
            TrainingInstance,
            "SELECT T1.id, T1.training_id, T2.name, T2.description, T1.weight, T1.times FROM training_instances AS T1
                JOIN trainings AS T2 ON T1.training_id = T2.id
                WHERE T1.task_id = $1",
            task.id,
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get training instances"))?;

        let reply = Reply {
            id: task.id,
            name: task.name,
            description: task.description,
            training_instances,
        };
        reply_all.push(reply)
    }

    Ok(reply_all)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_tasks")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
