use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub user_token: String,
    pub id: i64,
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
    pub weight_value: f64,
    pub count_value: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let task = sqlx::query!(
        "SELECT id, name, description FROM tasks
            WHERE id = $1 AND user_id = (SELECT id FROM users WHERE token = $2)",
        extract.id,
        extract.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get a task"))?;

    let training_instances = sqlx::query_as!(
        TrainingInstance,
        "SELECT T1.id, T1.training_id, T2.name, T2.description, T1.weight_value, T1.count_value FROM training_instances AS T1
            JOIN trainings AS T2 ON T1.training_id = T2.id
            WHERE T1.task_id = $1",
        task.id
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

    Ok(warp::reply::json(&reply))
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
