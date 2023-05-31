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

    let mut tx = db
        .begin()
        .await
        .map_err(|_| util::ErrorMessage::new("failed to begin transaction"))?;

    sqlx::query!(
        "INSERT INTO training_results (user_id, training_id, weight_value, count_value, done_at)
            SELECT (SELECT id FROM users WHERE token = $1), T2.training_id, T2.weight_value, T2.count_value, T1.done_at FROM tmp_training_results AS T1
            JOIN training_instances AS T2 ON T1.training_instance_id = T2.id
            WHERE T1.task_instance_id = $2",
        extract.user_token,
        extract.id,
    )
    .execute(&mut tx)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to delete a task instance"))?;

    sqlx::query!(
        "DELETE FROM task_instances
            WHERE id = $1 AND task_id IN (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $2))",
        extract.id,
        extract.user_token,
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
