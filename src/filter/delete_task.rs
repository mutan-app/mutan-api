use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
    pub id: i64,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<(), warp::Rejection> {
    let db = db.lock().await;

    // トークンが指すユーザを取得
    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    // タスクを取得
    let task = sqlx::query!("SELECT user_id FROM tasks WHERE id = $1", extract.id)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    // 他ユーザのタスクを取得した場合
    if task.user_id != user.id {
        return Err(util::error("no permission to access the task").into());
    }

    // タスクを削除
    sqlx::query!("DELETE FROM tasks WHERE id = $1", extract.id)
        .execute(&*db)
        .await
        .map_err(util::error)?;

    Ok(())
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("delete_task")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|_| warp::http::StatusCode::OK)
}
