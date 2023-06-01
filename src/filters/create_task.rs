use crate::filters::util;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
    pub name: String,
    pub description: Option<String>,
    pub trains: Vec<TrainingInstane>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TrainingInstane {
    pub train_id: i64,
    pub weight_val: f64,
    pub count_val: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT (id) FROM usr WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    let mut tx = db
        .begin()
        .await
        .map_err(|_| util::ErrorMessage::new("failed to begin transaction"))?;

    let task = sqlx::query!(
        "INSERT INTO task (usr_id, name, description) VALUES ($1, $2, $3) RETURNING id",
        user.id,
        extract.name,
        extract.description,
    )
    .fetch_one(&mut tx)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to create a task"))?;

    for (i, training_instance) in extract.trains.into_iter().enumerate() {
        sqlx::query!(
            "INSERT INTO train_ins (task_id, order_val, train_id, weight_val, count_val) VALUES ($1, $2, $3, $4, $5)",
            task.id,
            i as i32,
            training_instance.train_id,
            training_instance.weight_val,
            training_instance.count_val
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

    Ok(warp::http::StatusCode::OK)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_task")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
