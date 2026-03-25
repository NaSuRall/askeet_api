//use crate::config::AppState;
//use crate::models::Survey;
//use axum::{Json, extract::State};

//pub async fn liste_all_survey(State(state): State<AppState>) -> Json<Vec<Survey> {}

pub async fn liste_all_survey() -> &'static str {
    "Voici les surveys :"
}
