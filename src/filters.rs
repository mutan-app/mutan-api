use crate::{handlers, models};
use std::convert::Infallible;
use warp::Filter;

pub fn root(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    get_meta()
        .or(get_user(db.clone()))
        .or(create_user(db.clone())
        .or(delete_user(db.clone())))
}

pub fn get_meta() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("meta")
        .and(warp::get())
        .and_then(handlers::get_meta)
}

pub fn get_user(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_user")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::get_user)
}

pub fn create_user(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_user")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::create_user)
}

pub fn delete_user(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("delete_user")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::delete_user)
}

fn with_db(db: models::Db) -> impl Filter<Extract = (models::Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: std::marker::Send + serde::de::DeserializeOwned,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
