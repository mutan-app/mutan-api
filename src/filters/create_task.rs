use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
    pub name: String,
    pub description: Option<String>,
    pub training_instances: Vec<TrainingInstane>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TrainingInstane {
    pub training_id: i64,
    pub weight: f64,
    pub times: i32,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<Reply, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT (id) FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    let mut tx = db
        .begin()
        .await
        .map_err(|_| util::ErrorMessage::new("failed to begin transaction"))?;

    let task = sqlx::query!(
        "INSERT INTO tasks (user_id, name, description) VALUES ($1, $2, $3) RETURNING id",
        user.id,
        extract.name,
        extract.description,
    )
    .fetch_one(&mut tx)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to create a task"))?;

    for (idx, training_instance) in extract.training_instances.into_iter().enumerate() {
        sqlx::query!(
            "INSERT INTO training_instances (task_id, stage, training_id, weight, times) VALUES ($1, $2, $3, $4, $5)",
            task.id,
            idx as i32,
            training_instance.training_id,
            training_instance.weight,
            training_instance.times
        )
        .execute(&mut tx)
        .await
        .map_err(|_| {
            util::ErrorMessage::new("failed to add a training instance")
        })?;
    }

    tx.commit()
        .await
        .map_err(|_| util::ErrorMessage::new("failed to commit transaction"))?;

    let reply = Reply { id: task.id };

    Ok(reply)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_task")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
