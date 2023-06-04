use crate::util;
use std::borrow::Cow;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
    pub from: chrono::NaiveDateTime,
    pub to: chrono::NaiveDateTime,
    pub descending: bool,
    pub tag: Option<String>,
}

#[derive(Debug, Default, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Reply {
    pub at: Option<chrono::NaiveDateTime>,
    pub times: Option<i64>,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<Vec<Reply>, warp::Rejection> {
    let db = db.lock().await;

    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    let sort_dir = if extract.descending { "DESC" } else { "ASC" };

    let tag_cond = match extract.tag {
        Some(tag) => Cow::Owned(format!("'{}' = ANY(tags)", tag)),
        None => Cow::Borrowed("TRUE"),
    };

    let query = format!(
        "SELECT date_trunc('day', t1.done_at) AS at, COUNT(t1.id) AS times FROM training_results AS t1 LEFT JOIN trainings AS t2 ON t1.training_id = t2.id WHERE t1.user_id = $1 AND $2 <= t1.done_at AND t1.done_at < $3 AND {} GROUP BY at ORDER BY at {}",
        tag_cond, sort_dir,
    );

    let reply = sqlx::query_as::<_, Reply>(query.as_str())
        .bind(user.id)
        .bind(extract.from)
        .bind(extract.to)
        .fetch_all(&*db)
        .await
        .map_err(util::error)?;

    Ok(reply)
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_training_results")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
