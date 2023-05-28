mod create_task;
mod create_task_instance;
mod create_user;
mod delete_task;
mod delete_task_instance;
mod delete_user;
mod get_meta;
mod get_task;
mod get_task_instance;
mod get_task_instances;
mod get_tasks;
mod get_training;
mod get_trainings;
mod get_user;
mod proceed_task_instance;
mod util;

use warp::Filter;

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    get_meta::filter()
        .or(get_user::filter(db.clone()))
        .or(create_user::filter(db.clone()).or(delete_user::filter(db.clone())))
        .or(get_tasks::filter(db.clone()))
        .or(get_task::filter(db.clone()))
        .or(create_task::filter(db.clone()))
        .or(delete_task::filter(db.clone()))
        .or(get_task_instances::filter(db.clone()))
        .or(get_task_instance::filter(db.clone()))
        .or(create_task_instance::filter(db.clone()))
        .or(proceed_task_instance::filter(db.clone()))
        .or(delete_task_instance::filter(db.clone()))
        .or(get_trainings::filter(db.clone()))
        .or(get_training::filter(db))
}
