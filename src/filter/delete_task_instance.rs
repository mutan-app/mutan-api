use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<(), warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    let task_instance = sqlx::query!(
        "SELECT id, task_id, progress FROM task_instances WHERE task_id IN (SELECT id FROM tasks WHERE user_id = $1)",
        user.id
    )
    .fetch_one(&*db)
    .await
    .map_err(util::error)?;

    let mut tx = db.begin().await.map_err(util::error)?;

    let date_time = chrono::Utc::now().naive_utc();

    sqlx::query!(
        "INSERT INTO training_results (user_id, training_id, weight, times, done_at) SELECT $1, training_id, weight, times, $2 FROM training_instances WHERE task_id = $3 AND stage < $4",
        user.id,
        date_time,
        task_instance.task_id,
        task_instance.progress,
    )
    .execute(&mut tx)
    .await
    .map_err(util::error)?;

    sqlx::query!(
        "INSERT INTO task_results (task_id, done_at) VALUES ($1, $2)",
        task_instance.task_id,
        date_time,
    )
    .execute(&mut tx)
    .await
    .map_err(util::error)?;

    sqlx::query!("DELETE FROM task_instances WHERE id = $1", task_instance.id)
        .execute(&mut tx)
        .await
        .map_err(util::error)?;

    tx.commit().await.map_err(util::error)?;

    Ok(())
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("delete_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|_| warp::http::StatusCode::OK)
}
