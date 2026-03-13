use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::config::AppState;
use crate::models::AuthUser;
use crate::models::survey::{Survey, SurveyInsert};

// pub async fn survey_create(
//     State(state): State<AppState>,
//      // user: AuthUser,
//     Json(body): Json<SurveyInsert>,
// ) -> Json<Value> {
//
//     let id = Uuid::new_v4().to_string();
//     // let creator_id = user.id.to_string();
//     let creator_id = 2;
//
//     match sqlx::query!(
//         "INSERT INTO surveys (id, creator_id, title) VALUES (?, ?, ?)",
//         id,
//         creator_id,
//         body.title,
//     )
//         .execute(&state.db)
//         .await
//     {
//         Ok(_) => Json(json!({
//             "message": "Survey créé avec succès",
//             "status": "success"
//         })),
//         Err(e) => Json(json!({ "error": e.to_string() })),
//     }
// }

pub async fn survey_create()  {
    println!("caca");
}