use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub user_token: String,
    pub id: i64,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub task_id: i64,
    pub name: String,
    pub description: Option<String>,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let reply = sqlx::query_as!(
        Reply,
        "SELECT T1.id, T1.task_id, T2.name, T2.description FROM task_instances AS T1
            JOIN tasks AS T2 ON T1.task_id = T2.id
            WHERE T1.id = $1 AND T1.task_id = (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $2))",
        extract.id,
        extract.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get a task instance"))?;

    Ok(warp::reply::json(&reply))
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
