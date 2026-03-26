use crate::config::AppState;
use axum::{Json, extract::State};
use serde_json::{Value, json};
use crate::models::Survey;

pub async fn liste_all_survey(State(state): State<AppState>) -> Json<Value> {
    let surveys = sqlx::query_as::<_, Survey>(
        "SELECT id, creator_id, title, up, category_id FROM surveys"
    )
        .fetch_all(&state.db)
        .await;

    match surveys {
        Ok(surveys) => Json(json!({
            "status": "success",
            "data": surveys
        })),

        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}