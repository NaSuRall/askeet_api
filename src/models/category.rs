use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize,sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}