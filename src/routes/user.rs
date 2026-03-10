use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_users))
        .route("/{id}", get(get_user))
}

async fn list_users() -> &'static str {
    "Liste des utilisateurs"
}

async fn get_user() -> &'static str {
    "Détails d’un utilisateur"
}
