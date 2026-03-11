use crate::{config::AppState, models::User};
use axum::{Router, routing::get, Json};
use axum::routing::post;
use serde_json::{json, Value};
use crate::handlers::register;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/{id}", get(get_user))
        .route("/register", post(register::register))
}

// Utilisation du Model User avec le <User>
async fn list_users() -> &'static str {
    "Liste des utilisateurs"
}

async fn get_user() -> &'static str {
    "Détails d’un utilisateur"
}


