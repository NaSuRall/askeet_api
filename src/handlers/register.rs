use axum::{Json, extract::State};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{config::AppState, models::User};
use crate::models::RegisterUser;
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterUser>,
) -> Json<Value> {


    let id = Uuid::new_v4();

    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (id, last_name, first_name, pseudo, email, password) VALUES (?, ?, ?, ?, ?, ?)",
        id,
        body.last_name,
        body.first_name,
        body.pseudo,
        body.email,
        body.password,
    )
        .execute(&state.db)
        .await;


    match result {
        Ok(_) => Json(json!({
            "message": "User registered successfully",
            "status": "success"
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}