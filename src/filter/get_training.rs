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
    pub weight: f64,
    pub times: i32,
    pub tags: Vec<String>,
    pub done_times: i64,
    pub latest_done_at: Option<chrono::NaiveDateTime>,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<Reply, warp::Rejection> {
    let db = db.lock().await;

    // トークンが指すユーザを取得
    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    // トレーニング情報を取得
    let training = sqlx::query!(
        "SELECT t1.id, t1.name, t1.description, t1.weight, t1.times, t1.tags, COUNT(t2.id) AS done_times, MAX(t2.done_at) AS latest_done_at FROM trainings AS t1 LEFT JOIN training_results AS t2 ON t1.id = t2.training_id AND t2.user_id = $1 WHERE t1.id = $2 GROUP BY t1.id",
        user.id,
        extract.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(util::error)?;

    let reply = Reply {
        id: training.id,
        name: training.name,
        description: training.description,
        weight: training.weight,
        times: training.times,
        tags: training.tags,
        done_times: training.done_times.unwrap_or(0),
        latest_done_at: training.latest_done_at,
    };

    Ok(reply)
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_training")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
