use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub offset: i64,
    pub size: i64,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub weight: f64,
    pub times: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let reply = sqlx::query_as!(
        Reply,
        "SELECT id, name, description, weight, times FROM train OFFSET $1 LIMIT $2",
        extract.offset,
        extract.size,
    )
    .fetch_all(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get trainings"))?;

    Ok(warp::reply::json(&reply))
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_trainings")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
