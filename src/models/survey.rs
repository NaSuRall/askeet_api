use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,sqlx::FromRow)]
pub struct Survey {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub title: String,
    pub img: Option<String>,
    pub up: Option<i32>,
    pub color: Option<String>,
    pub category_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct CreateSurvey {
    pub creator_id: Uuid,
    pub title: String,
    pub img: Option<String>,
    pub category_id: Option<Uuid>
}
