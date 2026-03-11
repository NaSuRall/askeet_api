use axum::Json;
use serde_json::{json, Value};
use crate::models::User;

pub async fn register(Json(body): Json<User>) -> Json<Value>  {
    println!("body");
    Json(json!(body))
 }