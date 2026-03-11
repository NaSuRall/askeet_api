use crate::config::AppState;
use crate::models::LoginRequest;
use crate::models::User;
use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};

pub async fn login(State(state): State<AppState>, Json(body): Json<LoginRequest>) -> Json<Value> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password FROM users WHERE email = ?",
        body.email
    )
    .fetch_optional(&state.db)
    .await;

    match user {
        Ok(Some(user)) => {
            if user.password == body.password {
                // Function qui genere le token et qui envoie en json dans token
                // recupération de l'id avec le model User
                // et pour qu'il demande que l'email et mdp Model LoginRequest
                // je suis un genie
                let token = generate_token();
                Json(json!({
                    "token": token,
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
        })),
    }
}

pub fn generate_token() {
    println!("Bjr conanrd");
}
