use crate::config::AppState;
use crate::handlers::survey_handler;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/survey/all", get(survey_handler::liste_all_survey))
}
