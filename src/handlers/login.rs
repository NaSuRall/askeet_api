use crate::config::AppState;
use crate::models::{Claims, LoginRequest};
use crate::models::User;
use axum::Json;
use axum::extract::State;
use chrono::{Duration, Utc};
use serde_json::{Value, json};
use crate::models::claim;
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};


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
                let token = generate_token(user.id);
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

fn generate_token(user_id: u64) -> String {

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret_key".as_ref())
    ).unwrap()
}