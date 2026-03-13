use serde::{Deserialize, Serialize};
use serde_json::Number;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Survey{
    pub id: String,
    pub creator_id: String,
    pub title: String,
    pub img: Option<String>,
    pub up: Number,
    pub color: Option<String>,
    pub category_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SurveyInsert{
    pub id: String,
    pub creator_id: String,
    pub title: String,
}