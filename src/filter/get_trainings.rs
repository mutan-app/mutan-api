use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
    pub offset: i64,
    pub limit: i64,
    pub order_by: String,
    pub descending: bool,
    pub search: Option<String>,
    pub tag: Option<String>,
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

#[derive(Debug, Default, Clone, sqlx::FromRow)]
pub struct Training {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub weight: f64,
    pub times: i32,
    pub tags: Vec<String>,
    pub done_times: Option<i64>,
    pub latest_done_at: Option<chrono::NaiveDateTime>,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<Vec<Reply>, warp::Rejection> {
    let db = db.lock().await;

    // トークンが指すユーザを取得
    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    let sort_expr = match extract.order_by.as_str() {
        "name" => "t1.name",
        "times" => "done_times",
        "latest" => "latest_done_at",
        _ => return Err(util::error("invalid order by value").into()),
    };

    let sort_dir = if extract.descending { "DESC" } else { "ASC" };

    // ソート規則を動的に変更できるようクエリを構築
    let query = format!(
        "SELECT t1.id, t1.name, t1.description, t1.weight, t1.times, t1.tags, COUNT(t2.id) AS done_times, MAX(t2.done_at) AS latest_done_at FROM trainings AS t1 LEFT JOIN training_results AS t2 ON t1.id = t2.training_id AND t2.user_id = $1 WHERE ($4 IS NULL OR t1.name LIKE $4) AND ($5 IS NULL OR $5 = ANY(t1.tags)) GROUP BY t1.id ORDER BY {} {} OFFSET $2 LIMIT $3",
        sort_expr, sort_dir,
    );

    // 名前検索またはタグ検索で該当するトレーニング情報を取得
    let trainings = sqlx::query_as::<_, Training>(query.as_str())
        .bind(user.id)
        .bind(extract.offset)
        .bind(extract.limit)
        .bind(extract.search)
        .bind(extract.tag)
        .fetch_all(&*db)
        .await
        .map_err(util::error)?;

    let reply = trainings
        .into_iter()
        .map(|training| Reply {
            id: training.id,
            name: training.name,
            description: training.description,
            weight: training.weight,
            times: training.times,
            tags: training.tags,
            done_times: training.done_times.unwrap_or(0),
            latest_done_at: training.latest_done_at,
        })
        .collect::<Vec<_>>();

    Ok(reply)
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_trainings")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
