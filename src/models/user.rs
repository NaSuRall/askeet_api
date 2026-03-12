use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub last_name: String,
    pub first_name: String,
    pub pseudo: String,
    pub email: String,
    pub password: String,
    pub birth_date: Option<NaiveDate>,
    pub phone: Option<String>,
    pub pp: Option<String>,
    pub subscription: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub last_name: String,
    pub first_name: String,
    pub pseudo: String,
    pub email: String,
    pub password: String,
}