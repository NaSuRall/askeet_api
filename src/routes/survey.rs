use crate::config::AppState;
use crate::handlers::survey_handler;
use axum::{Router, routing::get};
use axum::routing::post;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/survey/all", get(survey_handler::liste_all_survey))
        .route("/survey/create", post(survey_handler::create_survey))
}
