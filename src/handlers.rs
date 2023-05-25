use crate::models;
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
        "select * from users where token = $1",
        json.token
    )
    .fetch_one(&*db)
    .await;

    let reply = user
        .map(|user| models::reply::GetUser { token: user.token })
        .ok();
    Ok(warp::reply::json(&reply))
}
