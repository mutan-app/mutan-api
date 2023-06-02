use crate::filters::util;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
    pub id: i64,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM usr WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a task"))?;

    let task_ins = sqlx::query!(
        "SELECT task_id, progress FROM task_ins
            WHERE id = $1 AND task_id IN (SELECT id FROM task WHERE usr_id = $2)",
        extract.id,
        user.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get a task"))?;

    let mut tx = db
        .begin()
        .await
        .map_err(|_| util::ErrorMessage::new("failed to begin transaction"))?;

    let date_time = chrono::Utc::now().naive_utc();

    sqlx::query!(
        "INSERT INTO train_res (usr_id, train_id, weight, times, done_at)
            SELECT $1, train_id, weight, times, $2 FROM train_ins
            WHERE task_id = $3 AND idx < $4",
        user.id,
        date_time,
        extract.id,
        task_ins.progress,
    )
    .execute(&mut tx)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to delete a task instance"))?;

    sqlx::query!(
        "INSERT INTO task_res (task_id, done_at) VALUES ($1, $2)",
        extract.id,
        date_time,
    )
    .execute(&mut tx)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to delete a task instance"))?;

    sqlx::query!(
        "DELETE FROM task_ins WHERE id = $1 AND task_id IN (SELECT id FROM task WHERE usr_id = $2)",
        extract.id,
        user.id,
    )
    .execute(&mut tx)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to delete a task instance"))?;

    tx.commit()
        .await
        .map_err(|_| util::ErrorMessage::new("failed to commit transaction"))?;

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
