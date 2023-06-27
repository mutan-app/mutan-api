use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
    pub task_id: i64,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<(), warp::Rejection> {
    let db = db.lock().await;

    // トークンがさすユーザを取得
    let user = sqlx::query!("SELECT id FROM users WHERE token = $1", extract.token)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    let task = sqlx::query!("SELECT user_id FROM tasks WHERE id = $1", extract.task_id)
        .fetch_one(&*db)
        .await
        .map_err(util::error)?;

    // 他ユーザのタスクを取得した場合
    if task.user_id != user.id {
        return Err(util::error("no permission to access the task").into());
    }

    // タスクインスタンスが存在する場合は取得
    let task_instance = sqlx::query!(
        "SELECT COUNT(id) FROM task_instances WHERE task_id IN (SELECT id FROM tasks WHERE user_id = $1)",
        user.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(util::error)?;

    // タスクインスタンスが1つ以上存在しないように
    let count = task_instance
        .count
        .ok_or_else(|| util::error("failed to count tasks"))?;
    if 0 < count {
        return Err(util::error("task instance already exists").into());
    }

    // タスクインスタンスを作成
    sqlx::query!(
        "INSERT INTO task_instances (task_id, progress) VALUES ($1, 0)",
        extract.task_id,
    )
    .execute(&*db)
    .await
    .map_err(util::error)?;

    Ok(())
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_task_instance")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|_| warp::http::StatusCode::OK)
}
