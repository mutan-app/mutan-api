use serde::Serialize;
use std::convert::Infallible;
use warp::Filter;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub name: String,
    pub version: String,
}

pub async fn handler() -> Result<Reply, Infallible> {
    let name = env!("CARGO_PKG_NAME").to_string();
    let version = env!("CARGO_PKG_VERSION").to_string();

    let reply = Reply { name, version };

    Ok(reply)
}

pub fn filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_meta")
        .and(warp::get())
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
