use axum::{routing::get, Router};
use std::net::SocketAddr;
mod controller;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(controller::hello_controller::hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
