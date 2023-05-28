use crate::filters::util;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub user_token: String,
    pub name: String,
    pub description: Option<String>,
    pub training_instances: Vec<TrainingInstane>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TrainingInstane {
    pub training_id: i64,
    pub weight_value: f64,
    pub count_value: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let task = sqlx::query!(
        "INSERT INTO tasks (name, description, user_id)
            VALUES ($1, $2, (SELECT id FROM users WHERE token = $3))
            RETURNING id",
        extract.name,
        extract.description,
        extract.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to create a task"))?;

    for (i, training_instance) in extract.training_instances.into_iter().enumerate() {
        sqlx::query!(
            "INSERT INTO training_instances (task_id, order_value, training_id, weight_value, count_value)
                VALUES ($1, $2, $3, $4, $5)",
            task.id,
            i as i32,
            training_instance.training_id,
            training_instance.weight_value,
            training_instance.count_value
        )
        .execute(&*db)
        .await
        .map_err(|_| {
            util::ErrorMessage::new("failed to add a training instance")
        })?;
    }

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
