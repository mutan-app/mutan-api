use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<(), warp::Rejection> {
    let db = db.lock().await;

    sqlx::query!("DELETE FROM users WHERE token = $1", extract.token)
        .execute(&*db)
        .await
        .map_err(util::error)?;

    Ok(())
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("delete_user")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|_| warp::http::StatusCode::OK)
}
