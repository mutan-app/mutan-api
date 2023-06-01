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
        "SELECT task_id, progress_val FROM task_ins
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

    sqlx::query!(
        "INSERT INTO train_res (usr_id, train_id, weight_val, count_val, done_at)
            SELECT $1, train_id, weight_val, count_val, $2 FROM train_ins
            WHERE task_id = $3 AND order_val < $4",
        user.id,
        chrono::Utc::now().naive_utc(),
        task_ins.task_id,
        task_ins.progress_val,
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
