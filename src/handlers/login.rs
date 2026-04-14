use crate::config::AppState;
use crate::models::{AuthUser, Claims, LoginRequest};
use axum::{Json, extract::State};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn login(State(state): State<AppState>, Json(body): Json<LoginRequest>) -> Json<Value> {
    let user = sqlx::query_as!(
        AuthUser,
        r#"
        SELECT
            id as "id: Uuid",
            email,
            password
        FROM users
        WHERE email = ?
        "#,
        body.email,
    )
    .fetch_optional(&state.db)
    .await;

    match user {
        Ok(Some(user)) => {
            // ✅ Vérification du mot de passe hashé
            if !verify(&body.password, &user.password).unwrap_or(false) {
                return Json(json!({
                    "status": "error",
                    "message": "Email ou mot de passe incorrect"
                }));
            }

            let token = generate_token(user.id.to_string());

            // ⚠️ On évite de renvoyer le password
            Json(json!({
                "token": token,
                "user": {
                    "id": user.id,
                    "email": user.email
                }
            }))
        }

        Ok(None) => Json(json!({
            "status": "error",
            "message": "Utilisateur non trouvé"
        })),

        Err(_) => Json(json!({
            "status": "error",
            "message": "Une erreur est survenue"
        })),
    }
}

fn generate_token(user_id: String) -> String {
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
        // ⚠️ À remplacer par une variable d'environnement en prod
        &EncodingKey::from_secret("secret_key".as_ref()),
    )
    .unwrap()
}

