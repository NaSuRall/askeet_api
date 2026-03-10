use axum::Router;

pub mod user;

pub fn create_router() -> Router {
    Router::new().nest("/user", user::routes())
}
