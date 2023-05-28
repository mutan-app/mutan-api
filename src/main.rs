mod filters;

#[tokio::main]
async fn main() {
    let port = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .ok()
        .and_then(|text| text.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, port);

    let url = std::env::var("DATABASE_URL").unwrap();
    let db = sqlx::PgPool::connect(&url).await.unwrap();
    let db = std::sync::Arc::new(tokio::sync::Mutex::new(db));

    warp::serve(filters::filter(db)).run(addr).await;
}
