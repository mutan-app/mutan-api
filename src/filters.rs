use crate::{handlers, models};
use std::convert::Infallible;
use warp::Filter;

pub fn root(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    get_meta()
        .or(get_user(db.clone()))
        .or(create_user(db.clone()).or(delete_user(db.clone())))
        .or(get_tasks(db.clone()))
        .or(get_task(db.clone()))
        .or(create_task(db.clone()))
        .or(delete_task(db.clone()))
        .or(get_task_instances(db.clone()))
        .or(get_task_instance(db.clone()))
        .or(create_task_instance(db.clone()))
        .or(proceed_task_instance(db.clone()))
        .or(delete_task_instance(db.clone()))
        .or(get_trainings(db.clone()))
        .or(get_training(db))
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

pub fn get_tasks(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_tasks")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::get_tasks)
}

pub fn get_task(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::get_task)
}

pub fn create_task(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_task")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_task)
}

pub fn delete_task(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_task")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::delete_task)
}

pub fn get_task_instances(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task_instances")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::get_task_instances)
}

pub fn get_task_instance(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_task_instance")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::get_task_instance)
}

pub fn create_task_instance(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_task_instance")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_task_instance)
}

pub fn proceed_task_instance(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("proceed_task_instance")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::proceed_task_instance)
}

pub fn delete_task_instance(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("delete_task_instance")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::delete_task_instance)
}

pub fn get_trainings(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_trainings")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::get_trainings)
}

pub fn get_training(
    db: models::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_training")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::get_training)
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
