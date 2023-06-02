use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub offset: i64,
    pub limit: i64,
    pub order_by: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub weight: f64,
    pub times: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<Vec<Reply>, warp::Rejection> {
    let db = db.lock().await;

    let reply = match extract.order_by.as_str() {
        "name" => sqlx::query_as!(
            Reply,
            "SELECT id, name, description, weight, times FROM trainings
                ORDER BY name ASC
                OFFSET $1 LIMIT $2",
            extract.offset,
            extract.limit,
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get trainings")),

        "times" => sqlx::query_as!(
            Reply,
            "SELECT id, name, description, weight, times FROM trainings
                ORDER BY (SELECT COUNT(id) FROM training_results WHERE training_id = id) DESC
                OFFSET $1 LIMIT $2",
            extract.offset,
            extract.limit,
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get trainings")),

        "recent" => sqlx::query_as!(
            Reply,
            "SELECT id, name, description, weight, times FROM trainings
                ORDER BY (SELECT MAX(done_at) FROM training_results WHERE training_id = id) DESC
                OFFSET $1 LIMIT $2",
            extract.offset,
            extract.limit,
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get trainings")),

        _ => Err(util::ErrorMessage::new("invalid order_by value")),
    }?;

    Ok(reply)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_trainings")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
