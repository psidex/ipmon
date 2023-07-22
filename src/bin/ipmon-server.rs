use std::net::SocketAddr;

use axum::{extract::ConnectInfo, http::header::HeaderMap, routing::get, Router};
use log::info;

use ipmon::platform;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let app = Router::new()
        .route("/", get(handle_root))
        .route("/health", get(handle_health));

    let ip = match platform::is_debug() {
        true => "127.0.0.1:8080",
        false => "0.0.0.0:8080",
    };

    axum::Server::bind(&ip.parse::<SocketAddr>().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn handle_health() {
    // Don't return anything, but not erring means 200 will be sent to client.
}

async fn handle_root(headers: HeaderMap, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    let client_ip;
    if headers.contains_key("X-Real-Ip") {
        client_ip = headers["X-Real-Ip"].to_str().unwrap().to_owned();
    } else if headers.contains_key("X-Forwarded-For") {
        client_ip = headers["X-Forwarded-For"].to_str().unwrap().to_owned();
    } else {
        client_ip = addr.to_string();
    }
    info!("Request from: {}", client_ip);
    client_ip
}
