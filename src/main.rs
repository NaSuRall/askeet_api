use axum::Router;
use sqlx::mysql::MySqlPoolOptions;
use crate::config::AppState;
use crate::routes::user::routes;

pub mod config;
mod models;
mod routes;

mod handlers;

#[tokio::main]
async fn main() {
    config::main();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL n'est pas set");

    let pool = MySqlPoolOptions::new().connect(&database_url).await.expect("ça marche pas idiot");

    let state = AppState {db:pool};

    let app = Router::new().nest("/users", routes().with_state(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
