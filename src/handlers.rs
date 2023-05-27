use crate::models;
use base64::Engine;
use rand::Rng;
use std::convert::Infallible;

pub async fn get_meta() -> Result<impl warp::Reply, Infallible> {
    let meta = models::reply::Meta {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(warp::reply::json(&meta))
}

pub async fn get_user(
    json: models::extract::GetUser,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    let user = sqlx::query_as!(
        models::User,
        "SELECT * FROM users WHERE token = $1",
        json.token
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    let reply = models::reply::GetUser { token: user.token };
    Ok(warp::reply::json(&reply))
}

pub async fn create_user(db: models::Db) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    let mut bytes = [0u8; 64];
    rand::thread_rng().fill(&mut bytes);
    let token = base64::engine::general_purpose::STANDARD.encode(bytes);

    sqlx::query!("INSERT INTO users (token) VALUES ($1)", token)
        .execute(&*db)
        .await
        .unwrap();

    let reply = models::reply::CreateUser { token };
    Ok(warp::reply::json(&reply))
}

pub async fn delete_user(
    json: models::extract::DeleteUser,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    sqlx::query!("DELETE FROM users WHERE token = $1", json.token)
        .execute(&*db)
        .await
        .unwrap();

    Ok(warp::http::StatusCode::OK)
}

pub async fn get_tasks(
    json: models::extract::GetTasks,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    let user = sqlx::query_as!(
        models::User,
        "SELECT * FROM users WHERE token = $1",
        json.user_token
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    let tasks = sqlx::query_as!(
        models::Task,
        "SELECT * FROM tasks WHERE user_id = $1 OFFSET $2 LIMIT $3",
        user.id,
        json.offset,
        json.count,
    )
    .fetch_all(&*db)
    .await
    .unwrap();

    let tasks = tasks
        .into_iter()
        .map(|task| models::reply::Task {
            name: task.name,
            description: task.description,
        })
        .collect::<Vec<_>>();
    let reply = models::reply::GetTasks { tasks };
    Ok(warp::reply::json(&reply))
}

pub async fn get_task(
    json: models::extract::GetTask,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    let user = sqlx::query_as!(
        models::User,
        "SELECT * FROM users WHERE token = $1",
        json.user_token,
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    let task = sqlx::query_as!(
        models::Task,
        "SELECT * FROM tasks WHERE id = $1 AND user_id = $2",
        json.id,
        user.id,
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    let task = models::reply::Task {
        name: task.name,
        description: task.description,
    };
    let reply = models::reply::GetTask { task };
    Ok(warp::reply::json(&reply))
}

pub async fn create_task(
    json: models::extract::CreateTask,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    let user = sqlx::query_as!(
        models::User,
        "SELECT * FROM users WHERE token = $1",
        json.user_token,
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    let task = sqlx::query_as!(
        models::Task,
        "INSERT INTO tasks (name, description, user_id) VALUES ($1, $2, $3) RETURNING *",
        json.name,
        json.description,
        user.id,
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    for (i, active_training) in json.active_trainings.into_iter().enumerate() {
        sqlx::query!(
            "INSERT INTO active_trainings (task_id, training_id, target_order, target_weight, target_count) VALUES ($1, $2, $3, $4, $5)",
            task.id,
            active_training.training_id,
            i as i32,
            active_training.weight,
            active_training.count
        )
        .execute(&*db)
        .await
        .unwrap();
    }

    Ok(warp::http::StatusCode::OK)
}

pub async fn delete_task(
    json: models::extract::DeleteTask,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    let user = sqlx::query_as!(
        models::User,
        "SELECT * FROM users WHERE token = $1",
        json.user_token,
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    sqlx::query!(
        "DELETE FROM tasks WHERE id = $1 AND user_id = $2",
        json.id,
        user.id
    )
    .execute(&*db)
    .await
    .unwrap();

    Ok(warp::http::StatusCode::OK)
}

pub async fn get_trainings(
    json: models::extract::GetTrainings,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    let trainings = sqlx::query_as!(
        models::Training,
        "SELECT * FROM trainings OFFSET $1 LIMIT $2",
        json.offset,
        json.count,
    )
    .fetch_all(&*db)
    .await
    .unwrap();

    let trainings = trainings
        .into_iter()
        .map(|training| models::reply::Training {
            name: training.name,
            description: training.description,
            default_weight: training.default_weight,
            default_count: training.default_count,
        })
        .collect::<Vec<_>>();
    let reply = models::reply::GetTrainings { trainings };
    Ok(warp::reply::json(&reply))
}

pub async fn get_training(
    json: models::extract::GetTraining,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;

    let training = sqlx::query_as!(
        models::Training,
        "SELECT * FROM trainings WHERE id = $1",
        json.id,
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    let training = models::reply::Training {
        name: training.name,
        description: training.description,
        default_weight: training.default_weight,
        default_count: training.default_count,
    };
    let reply = models::reply::GetTraining { training };
    Ok(warp::reply::json(&reply))
}
