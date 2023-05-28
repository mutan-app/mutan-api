use crate::filters::util;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub user_token: String,
    pub task_id: i64,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    sqlx::query!(
        "SELECT FROM tasks 
            WHERE id = $1 AND user_id = (SELECT id FROM users WHERE token = $2)",
        extract.task_id,
        extract.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get a task"))?;

    sqlx::query!(
        "INSERT INTO task_instances (task_id, progress_value)
            VALUES ($1, 0)",
        extract.task_id,
    )
    .execute(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to create a task instance"))?;

    Ok(warp::http::StatusCode::OK)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
