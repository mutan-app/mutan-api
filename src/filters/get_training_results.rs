use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
    pub offset: i64,
    pub limit: i64,
    pub order_by: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub training_id: i64,
    pub name: String,
    pub weight: f64,
    pub times: i32,
    pub done_at: chrono::NaiveDateTime,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<Vec<Reply>, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    let reply = match extract.order_by.as_str() {
        "new" => sqlx::query_as!(
            Reply,
            "SELECT T1.id, T1.training_id, T2.name, T1.weight, T1.times, T1.done_at FROM training_results AS T1
                JOIN trainings AS T2 ON T1.training_id = T2.id
                WHERE T1.user_id = $1
                ORDER BY id DESC
                OFFSET $2 LIMIT $3",
            user.id,
            extract.offset,
            extract.limit,
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get training results")),

        _ => Err(util::ErrorMessage::new("invalid order_by value")),
    }?;

    Ok(reply)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_training_results")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
