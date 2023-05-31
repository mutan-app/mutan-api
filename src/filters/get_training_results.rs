use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub user_token: String,
    pub offset: i64,
    pub size: i64,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub training_id: i64,
    pub name: String,
    pub weight_value: f64,
    pub count_value: i32,
    pub done_at: chrono::NaiveDateTime,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let reply = sqlx::query_as!(
        Reply,
        "SELECT T1.id, T1.training_id, T2.name, T1.weight_value, T1.count_value, T1.done_at FROM training_results AS T1
            JOIN trainings AS T2 ON T1.training_id = T2.id
            WHERE T1.user_id = (SELECT id FROM users WHERE token = $1)
            OFFSET $2 LIMIT $3",
        extract.user_token,
        extract.offset,
        extract.size,
    )
    .fetch_all(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get training_results"))?;

    Ok(warp::reply::json(&reply))
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_training_results")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
