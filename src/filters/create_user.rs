use crate::filters::util;
use base64::Engine;
use rand::Rng;
use serde::Serialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Extract {
    pub token: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub token: String,
}

pub async fn handler(db: util::Db) -> Result<Reply, warp::Rejection> {
    let mut bytes = [0u8; 64];
    rand::thread_rng().fill(&mut bytes);
    let token = base64::engine::general_purpose::STANDARD.encode(bytes);

    let db = db.lock().await;

    sqlx::query!("INSERT INTO users (token) VALUES ($1)", token)
        .execute(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to create a user"))?;

    let reply = Reply { token };

    Ok(reply)
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_user")
        .and(warp::get())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
