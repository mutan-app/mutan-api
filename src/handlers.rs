use crate::{models, rejects};
use base64::Engine;
use rand::Rng;
use std::convert::Infallible;

pub async fn get_meta() -> Result<impl warp::Reply, Infallible> {
    let name = env!("CARGO_PKG_NAME").to_string();
    let version = env!("CARGO_PKG_VERSION").to_string();

    let meta = models::reply::Meta { name, version };
    Ok(warp::reply::json(&meta))
}

pub async fn get_user(
    json: models::extract::GetUser,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let user = sqlx::query_as!(
        models::User,
        "SELECT * FROM users WHERE token = $1",
        json.token
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to create user"))?;

    let reply = models::reply::GetUser { token: user.token };
    Ok(warp::reply::json(&reply))
}

pub async fn create_user(db: models::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let mut bytes = [0u8; 64];
    rand::thread_rng().fill(&mut bytes);
    let token = base64::engine::general_purpose::STANDARD.encode(bytes);

    let db = db.lock().await;
    sqlx::query!("INSERT INTO users (token) VALUES ($1)", token)
        .execute(&*db)
        .await
        .map_err(|_| rejects::ErrorMessage::new("failed to create user"))?;

    let reply = models::reply::CreateUser { token };
    Ok(warp::reply::json(&reply))
}

pub async fn delete_user(
    json: models::extract::DeleteUser,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    sqlx::query!("DELETE FROM users WHERE token = $1", json.token)
        .execute(&*db)
        .await
        .map_err(|_| rejects::ErrorMessage::new("failed to delete user"))?;

    Ok(warp::http::StatusCode::OK)
}

pub async fn get_tasks(
    json: models::extract::GetTasks,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let tasks = sqlx::query_as!(
        models::Task,
        "SELECT * FROM tasks
            WHERE user_id = (SELECT id FROM users WHERE token = $1)",
        json.user_token,
    )
    .fetch_all(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to get tasks"))?;

    let reply_tasks = tasks
        .into_iter()
        .map(|task| models::reply::Task {
            name: task.name,
            description: task.description,
        })
        .collect::<Vec<_>>();
    let reply = models::reply::GetTasks { tasks: reply_tasks };
    Ok(warp::reply::json(&reply))
}

pub async fn get_task(
    json: models::extract::GetTask,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let task = sqlx::query_as!(
        models::Task,
        "SELECT * FROM tasks
            WHERE id = $1 AND user_id = (SELECT id FROM users WHERE token = $2)",
        json.id,
        json.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to get task"))?;

    let reply_task = models::reply::Task {
        name: task.name,
        description: task.description,
    };
    let reply = models::reply::GetTask { task: reply_task };
    Ok(warp::reply::json(&reply))
}

pub async fn create_task(
    json: models::extract::CreateTask,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let task = sqlx::query!(
        "INSERT INTO tasks (name, description, user_id)
            VALUES ($1, $2, (SELECT id FROM users WHERE token = $3))
            RETURNING (id)",
        json.name,
        json.description,
        json.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to create task"))?;

    for (i, training_instance) in json.training_instances.into_iter().enumerate() {
        sqlx::query!(
            "INSERT INTO training_instances (task_id, order_value, training_id, weight_value, count_value)
                VALUES ($1, $2, $3, $4, $5)",
            task.id,
            i as i32,
            training_instance.training_id,
            training_instance.weight_value,
            training_instance.count_vale
        )
        .execute(&*db)
        .await
        .map_err(|_| {
            rejects::ErrorMessage::new("failed to append training with task")
        })?;
    }

    Ok(warp::http::StatusCode::OK)
}

pub async fn delete_task(
    json: models::extract::DeleteTask,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    sqlx::query!(
        "DELETE FROM tasks
            WHERE id = $1 AND user_id = (SELECT id FROM users WHERE token = $2)",
        json.id,
        json.user_token,
    )
    .execute(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to delete task"))?;

    Ok(warp::http::StatusCode::OK)
}

pub async fn get_task_instances(
    json: models::extract::GetTaskInstance,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let task_instances = sqlx::query_as!(
        models::TaskInstance,
        "SELECT * FROM task_instances
            WHERE task_id = (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $1))",
        json.user_token,
    )
    .fetch_all(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to get task instances"))?;

    let reply_task_instances = task_instances
        .into_iter()
        .map(|task_instance| models::reply::TaskInstance {
            task_id: task_instance.task_id,
            progress_value: task_instance.progress_value,
        })
        .collect::<Vec<_>>();
    let reply = models::reply::GetTaskInstances {
        task_instances: reply_task_instances,
    };
    Ok(warp::reply::json(&reply))
}

pub async fn get_task_instance(
    json: models::extract::GetTaskInstance,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let task_instance = sqlx::query_as!(
        models::TaskInstance,
        "SELECT * FROM task_instances
            WHERE id = $1 AND task_id = (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $2))",
        json.id,
        json.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to get task instance"))?;

    let reply_task_instance = models::reply::TaskInstance {
        task_id: task_instance.task_id,
        progress_value: task_instance.progress_value,
    };
    let reply = models::reply::GetTaskInstance {
        task_instance: reply_task_instance,
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create_task_instance(
    json: models::extract::CreateTaskInstance,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    sqlx::query!(
        "SELECT FROM tasks 
            WHERE id = $1 AND user_id = (SELECT id FROM users WHERE token = $2)",
        json.task_id,
        json.user_token,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to get task"))?;
    sqlx::query!(
        "INSERT INTO task_instances (task_id, progress_value)
            VALUES ($1, $2)",
        json.task_id,
        json.progress_value,
    )
    .execute(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to create task instance"))?;

    Ok(warp::http::StatusCode::OK)
}

pub async fn proceed_task_instance(
    json: models::extract::ProceedTaskInstance,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    sqlx::query!(
        "UPDATE task_instances SET progress_value = $1 
            WHERE id = $2 AND task_id = (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $3))",
        json.progress_value,
        json.id,
        json.user_token,
    )
    .execute(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to update task instance"))?;

    Ok(warp::http::StatusCode::OK)
}

pub async fn delete_task_instance(
    json: models::extract::DeleteTaskInstance,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    sqlx::query!(
        "DELETE FROM task_instances
            WHERE id = $1 AND task_id = (SELECT id FROM tasks WHERE user_id = (SELECT id FROM users WHERE token = $2))",
        json.id,
        json.user_token,
    )
    .execute(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to delete task instance"))?;

    Ok(warp::http::StatusCode::OK)
}

pub async fn get_trainings(
    json: models::extract::GetTrainings,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let trainings = sqlx::query_as!(
        models::Training,
        "SELECT * FROM trainings OFFSET $1 LIMIT $2",
        json.offset,
        json.size,
    )
    .fetch_all(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to get trainings"))?;

    let trainings = trainings
        .into_iter()
        .map(|training| models::reply::Training {
            name: training.name,
            description: training.description,
            default_weight_value: training.default_weight_value,
            default_count_value: training.default_count_value,
        })
        .collect::<Vec<_>>();
    let reply = models::reply::GetTrainings { trainings };
    Ok(warp::reply::json(&reply))
}

pub async fn get_training(
    json: models::extract::GetTraining,
    db: models::Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let training = sqlx::query_as!(
        models::Training,
        "SELECT * FROM trainings WHERE id = $1",
        json.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| rejects::ErrorMessage::new("failed to get training"))?;

    let training = models::reply::Training {
        name: training.name,
        description: training.description,
        default_weight_value: training.default_weight_value,
        default_count_value: training.default_count_value,
    };
    let reply = models::reply::GetTraining { training };
    Ok(warp::reply::json(&reply))
}
