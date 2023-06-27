use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
    pub descending: bool,
    pub from: chrono::NaiveDateTime,
    pub to: chrono::NaiveDateTime,
    pub tag: Option<String>,
}

#[derive(Debug, Default, Clone, sqlx::FromRow)]
pub struct TimeSeries {
    pub at: Option<chrono::NaiveDateTime>,
    pub times: Option<i64>,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Reply {
    pub at: chrono::NaiveDateTime,
    pub times: i64,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<Vec<Reply>, warp::Rejection> {
    let db = db.lock().await;

    // トークンが指すユーザを取得
    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    let sort_dir = if extract.descending { "DESC" } else { "ASC" };

    // ソート順を動的に変更できるクエリを構築
    let query = format!(
        "SELECT date_trunc('day', t1.done_at) AS at, COUNT(t1.id) AS times FROM training_results AS t1 LEFT JOIN trainings AS t2 ON t1.training_id = t2.id WHERE t1.user_id = $1 AND $2 <= t1.done_at AND t1.done_at < $3 AND ($4 IS NULL OR $4 = ANY(t2.tags)) GROUP BY at ORDER BY at {}",
        sort_dir,
    );

    // 1日ごとのタグ別トレーニング完了数を取得(特定時期で切り取り)
    let time_series = sqlx::query_as::<_, TimeSeries>(query.as_str())
        .bind(user.id)
        .bind(extract.from)
        .bind(extract.to)
        .bind(extract.tag)
        .fetch_all(&*db)
        .await
        .map_err(util::error)?;

    // DB操作が正常に行われているか確認
    let reply = time_series
        .into_iter()
        .map(|time_series| {
            let at = time_series
                .at
                .ok_or_else(|| util::error("failed to group by truncated timestamp"))?;

            let times = time_series
                .times
                .ok_or_else(|| util::error("failed to group by truncated timestamp"))?;

            Ok(Reply { at, times })
        })
        .collect::<Result<Vec<_>, warp::Rejection>>()?;

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
