use axum::{Json, extract::State};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{config::AppState, models::User};

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<User>,
) -> Json<Value> {

    let id = Uuid::new_v4();
    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (id, name, email) VALUES (?, ?, ?) ",
        id.as_bytes(),
        body.name,
        body.email
    )
            .fetch_one(&state.db)
            .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_id(); // 👈 spécifique MySQL
            Json(json!({ "id": id, "name": body.name, "email": body.email }))
        }
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}