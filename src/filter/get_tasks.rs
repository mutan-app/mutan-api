use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
    pub offset: i64,
    pub limit: i64,
    pub order_by: String,
    pub descending: bool,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Reply {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub done_times: i64,
    pub latest_done_at: Option<chrono::NaiveDateTime>,
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

#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub done_times: Option<i64>,
    pub latest_done_at: Option<chrono::NaiveDateTime>,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<Vec<Reply>, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    let sort_expr = match extract.order_by.as_str() {
        "name" => "name",
        "times" => "done_times",
        "latest" => "latest_done_at",
        _ => return Err(util::error("invalid order by value").into()),
    };

    let sort_dir = if extract.descending { "DESC" } else { "ASC" };

    let query = format!(
        "SELECT t1.id, t1.name, t1.description, COUNT(t2.id) AS done_times, MAX(t2.done_at) AS latest_done_at FROM tasks AS t1 LEFT JOIN task_results AS t2 ON t1.id = t2.task_id WHERE user_id = $1 GROUP BY t1.id ORDER BY {} {} OFFSET $2 LIMIT $3",
        sort_expr, sort_dir,
    );

    let tasks = sqlx::query_as::<_, Task>(query.as_str())
        .bind(user.id)
        .bind(extract.offset)
        .bind(extract.limit)
        .fetch_all(&*db)
        .await
        .map_err(util::error)?;

    // not support async map
    let mut reply_all = vec![];
    for task in tasks {
        let training_instances = sqlx::query_as!(
            TrainingInstance,
            "SELECT t1.id, t1.training_id, t1.weight, t1.times, t2.name, t2.description, t2.tags FROM training_instances AS t1 LEFT JOIN trainings AS t2 ON t1.training_id = t2.id WHERE t1.task_id = $1",
            task.id,
        )
        .fetch_all(&*db)
        .await
        .map_err(util::error)?;

        let reply = Reply {
            id: task.id,
            name: task.name,
            description: task.description,
            done_times: task.done_times.unwrap_or(0),
            latest_done_at: task.latest_done_at,
            training_instances,
        };

        reply_all.push(reply)
    }

    Ok(reply_all)
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_tasks")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
