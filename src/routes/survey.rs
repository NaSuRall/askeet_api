use axum::Router;
use axum::routing::post;
use crate::config::AppState;
use crate::handlers::survey;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/survey/store", post(survey::survey_create))
}