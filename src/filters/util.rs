use std::convert::Infallible;
use warp::Filter;

pub type Db = std::sync::Arc<tokio::sync::Mutex<sqlx::PgPool>>;

pub fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
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
