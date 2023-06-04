use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
    pub progress: i32,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<(), warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    sqlx::query!(
        "UPDATE task_instances SET progress = $1 WHERE task_id IN (SELECT id FROM tasks WHERE user_id = $2)",
        extract.progress,
        user.id,
    )
    .execute(&*db)
    .await
    .map_err(util::error)?;

    Ok(())
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("proceed_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|_| warp::http::StatusCode::OK)
}
