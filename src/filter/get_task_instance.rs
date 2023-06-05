use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Reply {
    pub id: i64,
    pub task_id: i64,
    pub progress: i32,
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
    pub tags: Vec<String>,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<Reply, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    let task_instance = sqlx::query!(
        "SELECT t1.id, t1.task_id, t1.progress, t2.user_id, t2.name, t2.description FROM task_instances AS t1 LEFT JOIN tasks AS t2 ON t1.task_id = t2.id WHERE t1.task_id IN (SELECT id FROM tasks WHERE user_id = $1)",
        user.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(util::error)?;

    // prevent to access other user's task
    if task_instance.user_id != user.id {
        return Err(util::error("no permission to access the task").into());
    }

    let training_instances = sqlx::query_as!(
        TrainingInstance,
        "SELECT t1.id, t1.training_id, t1.weight, t1.times, t2.name, t2.description, t2.tags FROM training_instances AS t1 LEFT JOIN trainings AS t2 ON t1.training_id = t2.id WHERE t1.task_id = $1",
        task_instance.task_id
    )
    .fetch_all(&*db)
    .await
        .map_err(util::error)?;

    let reply = Reply {
        id: task_instance.id,
        task_id: task_instance.task_id,
        name: task_instance.name,
        description: task_instance.description,
        training_instances,
        progress: task_instance.progress,
    };

    Ok(reply)
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
