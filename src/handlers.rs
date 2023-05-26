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
    .await;

    let reply = user
        .map(|user| models::reply::GetUser { token: user.token })
        .ok();
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
