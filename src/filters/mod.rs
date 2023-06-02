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
mod get_training_results;
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
        .or(get_training::filter(db.clone()))
        .or(get_training_results::filter(db))
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn connect_db() -> std::sync::Arc<tokio::sync::Mutex<sqlx::PgPool>> {
        let url = std::env::var("DATABASE_URL").unwrap();
        let db = sqlx::PgPool::connect(&url).await.unwrap();
        std::sync::Arc::new(tokio::sync::Mutex::new(db))
    }

    #[tokio::test]
    async fn create_get_delete_user() {
        let db = connect_db().await;

        let new_user = create_user::handler(db.clone()).await.unwrap();

        let user = get_user::handler(
            get_user::Extract {
                token: new_user.token.clone(),
            },
            db.clone(),
        )
        .await
        .unwrap();

        assert_eq!(new_user.token, user.token);

        delete_user::handler(
            delete_user::Extract {
                token: user.token.clone(),
            },
            db.clone(),
        )
        .await
        .unwrap();

        get_user::handler(get_user::Extract { token: user.token }, db.clone())
            .await
            .unwrap_err();
    }

    #[tokio::test]
    async fn get_training() {
        let db = connect_db().await;

        let trainings = get_trainings::handler(
            get_trainings::Extract {
                offset: 0,
                size: 20,
            },
            db.clone(),
        )
        .await
        .unwrap();

        for training_entry in trainings {
            let training = get_training::handler(
                get_training::Extract {
                    id: training_entry.id,
                },
                db.clone(),
            )
            .await
            .unwrap();
            assert_eq!(training.id, training_entry.id);
        }
    }

    #[tokio::test]
    async fn create_get_delete_task() {
        let db = connect_db().await;

        let user = create_user::handler(db.clone()).await.unwrap();

        let new_task = create_task::handler(
            create_task::Extract {
                token: user.token.clone(),
                name: "New Task".into(),
                description: None,
                trains: vec![],
            },
            db.clone(),
        )
        .await
        .unwrap();

        let task = get_task::handler(
            get_task::Extract {
                token: user.token.clone(),
                id: new_task.id,
            },
            db.clone(),
        )
        .await
        .unwrap();

        assert_eq!(new_task.id, task.id);

        delete_task::handler(
            delete_task::Extract {
                token: user.token.clone(),
                id: task.id,
            },
            db.clone(),
        )
        .await
        .unwrap();

        get_task::handler(
            get_task::Extract {
                token: user.token.clone(),
                id: new_task.id,
            },
            db.clone(),
        )
        .await
        .unwrap_err();

        delete_user::handler(delete_user::Extract { token: user.token }, db)
            .await
            .unwrap();
    }
}
