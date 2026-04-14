use axum::Router;
use axum::routing::get;
use crate::config::AppState;
use crate::handlers::category_handler;

pub fn routes() -> Router<AppState> {
    Router::new().route("/category/all", get(category_handler::liste_all_categories))
}