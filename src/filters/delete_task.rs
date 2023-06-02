use crate::filters::util;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
    pub id: i64,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<(), warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM usr WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    sqlx::query!(
        "DELETE FROM task WHERE id = $1 AND usr_id = $2",
        extract.id,
        user.id,
    )
    .execute(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to delete a task"))?;

    Ok(())
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("delete_task")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|_| warp::http::StatusCode::OK)
}
