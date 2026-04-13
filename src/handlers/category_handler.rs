use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use crate::config::AppState;
use crate::models::category::Category;

pub async fn liste_all_categories(State(state): State<AppState>) ->Json<Value>{
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id , name FROM categories ORDER BY id ASC"
    ).fetch_all(&state.db).await;

    match categories {
        Ok(categories) => Json(json!({
            "status": "success",
            "data": categories
        })),

        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}