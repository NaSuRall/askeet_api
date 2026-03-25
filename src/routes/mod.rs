use crate::config::AppState;
use axum::Router;
pub mod survey;
pub mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/api", user::routes())
        .nest("/api", survey::routes())
}
