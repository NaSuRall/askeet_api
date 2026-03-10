use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;

use crate::routes;

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL non trouver");

    let address = env::var("SERVER_ADDRESS").unwrap();
    let port = env::var("SERVER_PORT").unwrap();
    let server_addr = format!("{}:{}", address, port);

    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();

    println!("Server running on {}", server_addr);

    axum::serve(listener, app).await.unwrap();
}
