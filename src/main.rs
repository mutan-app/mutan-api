mod filter;
mod util;

#[tokio::main]
async fn main() {
    let port = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .ok()
        .and_then(|text| text.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, port);

    let db = util::new_app_db().await.unwrap();

    warp::serve(filter::filter(db)).run(addr).await;
}
