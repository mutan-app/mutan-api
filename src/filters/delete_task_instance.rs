use crate::filters::util;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub user_token: String,
    pub id: i64,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    sqlx::query!(
        "DELETE FROM task_instances
            WHERE id = $1 AND task_id = (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $2))",
        extract.id,
        extract.user_token,
    )
    .execute(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to delete a task instance"))?;

    Ok(warp::http::StatusCode::OK)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("delete_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
