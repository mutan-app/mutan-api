use crate::filters::util;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub user_token: String,
    pub id: i64,
    pub progress_value: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let task_instance = sqlx::query!(
        "SELECT task_id, progress_value FROM task_instances
            WHERE id = $1 AND task_id IN (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $2))",
        extract.id,
        extract.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get a task instance"))?;

    let mut tx = db
        .begin()
        .await
        .map_err(|_| util::ErrorMessage::new("failed to begin transaction"))?;

    let (prev, next) = (task_instance.progress_value, extract.progress_value);

    let timestamp = chrono::Utc::now().naive_utc();
    for i in prev..next {
        sqlx::query!(
            "INSERT INTO tmp_training_results (task_instance_id, training_instance_id, done_at)
                VALUES ($1, (SELECT id FROM training_instances WHERE task_id = $2 AND order_value = $3), $4)",
            extract.id,
            task_instance.task_id,
            i,
            timestamp,
        )
        .execute(&mut tx)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to create temporary training result"))?;
    }

    for i in next..prev {
        sqlx::query!(
            "DELETE FROM tmp_training_results
                WHERE task_instance_id = $1 AND training_instance_id = (SELECT id FROM training_instances WHERE task_id = $2 AND order_value = $3)",
            extract.id,
            task_instance.task_id,
            i,
        )
        .execute(&mut tx)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to delete temporary training result"))?;
    }

    sqlx::query!(
        "UPDATE task_instances SET progress_value = $1 
            WHERE id = $2 AND task_id IN (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $3))",
        extract.progress_value,
        extract.id,
        extract.user_token,
    )
    .execute(&mut tx)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to update a task instance"))?;

    tx.commit()
        .await
        .map_err(|_| util::ErrorMessage::new("failed to commit transaction"))?;

    Ok(warp::http::StatusCode::OK)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("proceed_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
