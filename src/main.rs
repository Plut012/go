use axum::{
    Router,
    routing::get,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

mod game;
mod ws;
mod state;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws::handler))
        .nest_service("/themes", ServeDir::new("themes"))
        .nest_service("/", ServeDir::new("frontend/dist"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
