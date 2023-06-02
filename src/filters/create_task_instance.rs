use crate::filters::util;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
    pub task_id: i64,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<(), warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    let task = sqlx::query!("SELECT user_id FROM tasks WHERE id = $1", extract.task_id)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a task"))?;

    if task.user_id != user.id {
        return Err(util::ErrorMessage::new("failed to get a task").into());
    }

    let task_instance = sqlx::query!(
        "SELECT COUNT(id) FROM task_instances
            WHERE task_id IN (SELECT id FROM tasks WHERE user_id = $1)",
        user.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get tasks"))?;

    let count = task_instance
        .count
        .ok_or_else(|| util::ErrorMessage::new("failed to count tasks"))?;
    if 0 < count {
        return Err(util::ErrorMessage::new("failed to create a task instance").into());
    }

    sqlx::query!(
        "INSERT INTO task_instances (task_id, progress) VALUES ($1, 0)",
        extract.task_id,
    )
    .execute(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to create a task instance"))?;

    Ok(())
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|_| warp::http::StatusCode::OK)
}
