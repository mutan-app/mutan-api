use std::convert::Infallible;
use warp::Filter;

pub type AppDb = std::sync::Arc<tokio::sync::Mutex<sqlx::PgPool>>;

pub async fn new_app_db() -> Result<AppDb, warp::Rejection> {
    let url = std::env::var("DATABASE_URL").map_err(|_| error("failed to get DATABASE_URL"))?;
    let db = sqlx::PgPool::connect(&url)
        .await
        .map_err(|_| error("failed to connect db"))?;
    Ok(std::sync::Arc::new(tokio::sync::Mutex::new(db)))
}

pub fn with_db(db: AppDb) -> impl Filter<Extract = (AppDb,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: std::marker::Send + serde::de::DeserializeOwned,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Debug)]
pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    pub fn new<M>(message: M) -> Self
    where
        M: std::fmt::Display,
    {
        let message = message.to_string();
        Self { message }
    }
}

impl warp::reject::Reject for ErrorMessage {}

pub fn error<M>(message: M) -> ErrorMessage
where
    M: std::fmt::Display,
{
    ErrorMessage::new(message)
}
