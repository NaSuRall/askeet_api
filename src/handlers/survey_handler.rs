use crate::config::AppState;
use axum::{Json, extract::State};
use serde_json::{Value, json};
use uuid::Uuid;
use crate::models::Survey;
use crate::models::CreateSurvey;

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

pub async fn create_survey(State(state): State<AppState>, Json(body): Json<CreateSurvey>) -> Json<Value> {

    let id = Uuid::new_v4();

    let survey = sqlx::query(
        "INSERT INTO surveys (id,creator_id, title, img, category_id) VALUES (?, ?, ?, ?, ?)"
    )
        .bind(id.to_string())
        .bind(body.creator_id)
        .bind(body.title)
        .bind(body.img)
        .bind(body.category_id)
        .execute(&state.db)
        .await;

    match survey {
         Ok(_) => Json(json!({
            "meassage": "tata",
            "status": "sucess"
        })),
        Err(e) => Json(json!({ "status": e.to_string() }))
    }

}

