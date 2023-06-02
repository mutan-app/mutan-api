use crate::filters::*;

async fn connect_db() -> std::sync::Arc<tokio::sync::Mutex<sqlx::PgPool>> {
    let url = std::env::var("DATABASE_URL").unwrap();
    let db = sqlx::PgPool::connect(&url).await.unwrap();
    std::sync::Arc::new(tokio::sync::Mutex::new(db))
}

#[tokio::test]
async fn crud_user() {
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
            limit: 20,
            order_by: "name".into(),
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
async fn crud_task() {
    let db = connect_db().await;

    let user = create_user::handler(db.clone()).await.unwrap();

    let new_task = create_task::handler(
        create_task::Extract {
            token: user.token.clone(),
            name: "New Task".into(),
            description: None,
            training_instances: vec![
                create_task::TrainingInstane {
                    training_id: 1,
                    weight: 30.0,
                    times: 5,
                },
                create_task::TrainingInstane {
                    training_id: 2,
                    weight: 60.0,
                    times: 10,
                },
            ],
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

#[tokio::test]
async fn crud_task_instance() {
    let db = connect_db().await;

    let user = create_user::handler(db.clone()).await.unwrap();

    let task = create_task::handler(
        create_task::Extract {
            token: user.token.clone(),
            name: "New Task".into(),
            description: None,
            training_instances: vec![
                create_task::TrainingInstane {
                    training_id: 1,
                    weight: 30.0,
                    times: 5,
                },
                create_task::TrainingInstane {
                    training_id: 2,
                    weight: 60.0,
                    times: 10,
                },
            ],
        },
        db.clone(),
    )
    .await
    .unwrap();

    create_task_instance::handler(
        create_task_instance::Extract {
            token: user.token.clone(),
            task_id: task.id,
        },
        db.clone(),
    )
    .await
    .unwrap();

    proceed_task_instance::handler(
        proceed_task_instance::Extract {
            token: user.token.clone(),
            progress: 2,
        },
        db.clone(),
    )
    .await
    .unwrap();

    delete_task_instance::handler(
        delete_task_instance::Extract {
            token: user.token.clone(),
        },
        db.clone(),
    )
    .await
    .unwrap();

    let training_results = get_training_results::handler(
        get_training_results::Extract {
            token: user.token.clone(),
            offset: 0,
            limit: 20,
            order_by: "new".into(),
        },
        db.clone(),
    )
    .await
    .unwrap();

    for training_result in training_results {
        if training_result.training_id == 1 {
            assert_eq!(training_result.weight, 30.0);
            assert_eq!(training_result.times, 5);
        } else if training_result.training_id == 2 {
            assert_eq!(training_result.weight, 60.0);
            assert_eq!(training_result.times, 10);
        } else {
            unreachable!();
        }
    }

    delete_task::handler(
        delete_task::Extract {
            token: user.token.clone(),
            id: task.id,
        },
        db.clone(),
    )
    .await
    .unwrap();

    delete_user::handler(delete_user::Extract { token: user.token }, db)
        .await
        .unwrap();
}

#[tokio::test]
async fn get_training_result() {
    let db = connect_db().await;

    let user = create_user::handler(db.clone()).await.unwrap();

    get_training_results::handler(
        get_training_results::Extract {
            token: user.token.clone(),
            offset: 0,
            limit: 20,
            order_by: "new".into(),
        },
        db.clone(),
    )
    .await
    .unwrap();

    delete_user::handler(delete_user::Extract { token: user.token }, db)
        .await
        .unwrap();
}
