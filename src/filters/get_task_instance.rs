use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub task_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub training_instances: Vec<TrainingInstance>,
    pub progress: i32,
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

pub async fn handler(extract: Extract, db: util::Db) -> Result<Reply, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    let task_instance = sqlx::query!(
        "SELECT T1.id, T1.task_id, T2.name, T2.description, T1.progress, T2.user_id FROM task_instances AS T1
            JOIN tasks AS T2 ON T1.task_id = T2.id
            WHERE T1.task_id IN (SELECT id FROM tasks WHERE id = $1)",
        user.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get a task instance"))?;

    if task_instance.user_id != user.id {
        return Err(util::ErrorMessage::new("failed to get a task instance").into());
    }

    let training_instances = sqlx::query_as!(
        TrainingInstance,
        "SELECT T1.id, T1.training_id, T2.name, T2.description, T1.weight, T1.times FROM training_instances AS T1
            JOIN trainings AS T2 ON T1.training_id = T2.id
            WHERE T1.task_id = $1",
        task_instance.task_id
    )
    .fetch_all(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get training instances"))?;

    let reply = Reply {
        id: task_instance.id,
        task_id: task_instance.task_id,
        name: task_instance.name,
        description: task_instance.description,
        training_instances,
        progress: task_instance.progress,
    };

    Ok(reply)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
