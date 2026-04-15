use crate::config::AppState;
use axum::Router;
pub mod survey;
pub mod user;
pub mod category;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/api",
            user::routes().merge(survey::routes()).merge(category::routes())
        )
}
