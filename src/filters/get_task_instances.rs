use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub token: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub task_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub trains: Vec<Train>,
    pub progress: i32,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Train {
    pub id: i64,
    pub train_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub weight: f64,
    pub times: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM usr WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get a user"))?;

    let tasks = sqlx::query!(
        "SELECT T1.id, T1.task_id, T2.name, T2.description, T1.progress FROM task_ins AS T1
            JOIN task AS T2 ON T1.task_id = T2.id
            WHERE T1.task_id IN (SELECT id FROM task WHERE usr_id = $1)",
        user.id,
    )
    .fetch_all(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get task instances"))?;

    let mut reply_all = vec![];
    for task in tasks {
        let trains = sqlx::query_as!(
            Train,
            "SELECT T1.id, T1.train_id, T2.name, T2.description, T1.weight, T1.times FROM train_ins AS T1
                JOIN train AS T2 ON T1.train_id = T2.id
                WHERE T1.task_id = $1",
            task.task_id
        )
        .fetch_all(&*db)
        .await
        .map_err(|_| util::ErrorMessage::new("failed to get training instances"))?;

        let reply = Reply {
            id: task.id,
            task_id: task.task_id,
            name: task.name,
            description: task.description,
            trains,
            progress: task.progress,
        };
        reply_all.push(reply);
    }

    Ok(warp::reply::json(&reply_all))
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task_instances")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
