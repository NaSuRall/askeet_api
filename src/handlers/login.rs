use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use crate::models::User;
use crate::config::AppState;

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<User>) -> Json<Value>  {

    let user = sqlx::query_as!(
        User,
        "SELECT email, password FROM users WHERE email = ?",
        body.email
    )
        .fetch_optional(&state.db)
        .await;

    match user {
        Ok(Some(user)) => {
            if user.password == body.password {
                // Function qui genere le token et qui envoie en json dans token
                Json(json!({
                    "token": "success",
                    "user": user
                }))
            } else {
                Json(json!({
                    "status": "error",
                    "message": "Email ou mot de passe incorect"
                }))
            }
        }

        Ok(None) => Json(json!({
            "status": "error",
            "message": "Utilisateur non trouver"
        })),

        Err(_) => Json(json!({
            "status": "error",
            "message": "Une erreur est survenue"
        }))
    }

}