mod models;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .ok()
        .and_then(|text| text.parse().ok())
        .unwrap_or(3000);

    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, port);

    use warp::Filter;
    let version = warp::path!("version")
        .map(|| format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));

    warp::serve(version).run(addr).await;
}
