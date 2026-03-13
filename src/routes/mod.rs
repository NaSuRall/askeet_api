use crate::config::AppState;
use axum::Router;
pub mod user;
pub mod survey;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/api", user::routes()).nest("/api", survey::route())
}
