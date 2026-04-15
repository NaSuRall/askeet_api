use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::{config::AppState, models::user::UserResponse};
use crate::models::claim::Claims;
use uuid::Uuid;


pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, StatusCode> {
    
    println!("TA MERE");
    // 1. Récupérer le header Authorization
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. Extraire le token (enlever "Bearer ")
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 3. Décoder le JWT
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET manquant");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = token_data.claims.sub;
    println!("User ID extrait du token : {}", user_id);
    // 4. Charger le user depuis la DB
    println!("TA MERE 4");
    println!("user_id = {}", user_id);

    let user = sqlx::query_as::<_, UserResponse>(
        "SELECT LOWER(CONCAT(
            SUBSTR(HEX(id), 1, 8), '-',
            SUBSTR(HEX(id), 9, 4), '-',
            SUBSTR(HEX(id), 13, 4), '-',
            SUBSTR(HEX(id), 17, 4), '-',
            SUBSTR(HEX(id), 21)
        )) as id, last_name, first_name, pseudo, email, birth_date, phone, pp, subscription,
        CAST(created_at AS DATETIME) as created_at
        FROM users WHERE id = UNHEX(REPLACE(?, '-', ''))"
    )
    .bind(&user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        println!("Erreur SQL : {:?}", e);
        StatusCode::NOT_FOUND
    })?;

    println!("TA MERE 5");
    Ok(Json(user))  
}
