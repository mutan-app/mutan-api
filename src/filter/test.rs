use crate::filter::*;
use crate::util;

#[tokio::test]
async fn crud_user() {
    let db = util::new_app_db().await.unwrap();

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
    let db = util::new_app_db().await.unwrap();

    let user = create_user::handler(db.clone()).await.unwrap();

    let trainings = get_trainings::handler(
        get_trainings::Extract {
            token: user.token.clone(),
            offset: 0,
            limit: 20,
            order_by: "name".into(),
            descending: false,
            search: None,
            tag: Some("腕".into()),
        },
        db.clone(),
    )
    .await
    .unwrap();

    for training_entry in trainings {
        let training = get_training::handler(
            get_training::Extract {
                token: user.token.clone(),
                id: training_entry.id,
            },
            db.clone(),
        )
        .await
        .unwrap();
        assert_eq!(training.id, training_entry.id);
    }

    delete_user::handler(delete_user::Extract { token: user.token }, db)
        .await
        .unwrap();
}

#[tokio::test]
async fn crud_task() {
    let db = util::new_app_db().await.unwrap();

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
    let db = util::new_app_db().await.unwrap();

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

    let time_series = get_time_series::handler(
        get_time_series::Extract {
            token: user.token.clone(),
            from: chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            to: chrono::NaiveDate::from_ymd_opt(3000, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            descending: false,
            tag: Some("腕".into()),
        },
        db.clone(),
    )
    .await
    .unwrap();

    assert_eq!(time_series.len(), 1);

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
async fn get_tasks() {
    let db = util::new_app_db().await.unwrap();

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

    get_tasks::handler(
        get_tasks::Extract {
            token: user.token.clone(),
            offset: 0,
            limit: 20,
            order_by: "times".into(),
            descending: false,
        },
        db.clone(),
    )
    .await
    .unwrap();

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
