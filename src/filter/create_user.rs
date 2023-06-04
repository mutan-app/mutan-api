use crate::util;
use base64::Engine;
use rand::Rng;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Reply {
    pub token: String,
}

pub async fn handler(db: util::AppDb) -> Result<Reply, warp::Rejection> {
    let mut bytes = [0u8; 64];
    rand::thread_rng().fill(&mut bytes);
    let token = base64::engine::general_purpose::STANDARD.encode(bytes);

    let db = db.lock().await;

    sqlx::query!("INSERT INTO users (token) VALUES ($1)", token)
        .execute(&*db)
        .await
        .map_err(util::error)?;

    let reply = Reply { token };

    Ok(reply)
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_user")
        .and(warp::get())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
