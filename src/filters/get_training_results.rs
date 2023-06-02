use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
    pub offset: i64,
    pub size: i64,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub train_id: i64,
    pub name: String,
    pub weight: f64,
    pub times: i32,
    pub done_at: chrono::NaiveDateTime,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM usr WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    let reply = sqlx::query_as!(
        Reply,
        "SELECT T1.id, T1.train_id, T2.name, T1.weight, T1.times, T1.done_at FROM train_res AS T1
            JOIN train AS T2 ON T1.train_id = T2.id
            WHERE T1.usr_id = $1
            OFFSET $2 LIMIT $3",
        user.id,
        extract.offset,
        extract.size,
    )
    .fetch_all(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get training results"))?;

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
