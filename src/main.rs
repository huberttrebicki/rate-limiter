use std::net::SocketAddr;

use axum::{
    Router,
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use rate_limiter::RateLimiter;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let limiter = RateLimiter::new();

    let app = Router::new()
        .route("/", get(handle_request))
        .with_state(limiter);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind TCP listener");

    println!("listening on http://127.0.0.1:3000");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("server failed");
}

async fn handle_request(
    State(limiter): State<RateLimiter>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    match limiter.allow(&addr.ip().to_string()) {
        Ok(()) => (StatusCode::OK, format!("allowed for {}\n", addr.ip())),
        Err(err) => (StatusCode::TOO_MANY_REQUESTS, format!("blocked: {err}\n")),
    }
}
