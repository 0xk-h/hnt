use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a number");

    let app = Router::new()
        .nest("/api", routes::message::router())
        .layer(CorsLayer::new().allow_origin(Any));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
