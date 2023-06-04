use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
    pub id: i64,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Reply {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub training_instances: Vec<TrainingInstance>,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct TrainingInstance {
    pub id: i64,
    pub training_id: i64,
    pub weight: f64,
    pub times: i32,
    pub name: String,
    pub description: Option<String>,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<Reply, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    let task = sqlx::query!(
        "SELECT id, name, description, user_id FROM tasks WHERE id = $1",
        extract.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(util::error)?;

    // prevent to access other user's task
    if task.user_id != user.id {
        return Err(util::error("no permission to access the task").into());
    }

    let training_instances = sqlx::query_as!(
        TrainingInstance,
        "SELECT t1.id, t1.training_id, t1.weight, t1.times, t2.name, t2.description FROM training_instances AS t1 LEFT JOIN trainings AS t2 ON t1.training_id = t2.id WHERE t1.task_id = $1",
        task.id
    )
    .fetch_all(&*db)
    .await
        .map_err(util::error)?;

    let reply = Reply {
        id: task.id,
        name: task.name,
        description: task.description,
        training_instances,
    };

    Ok(reply)
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
