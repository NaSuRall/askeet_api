use crate::{config::AppState, models::User};
use axum::{Router, routing::get};
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/{id}", get(get_user))
}

// Utilisation du Model User avec le <User>
async fn list_users() -> &'static str {
    "Liste des utilisateurs"
}

async fn get_user() -> &'static str {
    "Détails d’un utilisateur"
}
