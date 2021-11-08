mod echo_server;
mod proxy;
mod test;

#[tokio::main]
async fn main() {
    let args = std::env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:8080".to_string());
    let serv_addr: std::net::SocketAddr = args.parse().unwrap();

    tracing_subscriber::fmt()
        .with_target(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    // start echo server
    tokio::spawn(echo_server::start_server(args.clone()));

    tokio::spawn(proxy::start_proxy("127.0.0.1:8070".to_string(), serv_addr))
        .await
        .unwrap();
}
