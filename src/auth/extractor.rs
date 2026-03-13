// permet de recuperer l'id du user a partir de son token

use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthUser {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {

        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or((axum::http::StatusCode::UNAUTHORIZED, "No auth header".into()))?
            .to_str()
            .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Invalid header".into()))?;

        let token = auth_header.replace("Bearer ", "");

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
            .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Invalid uuid".into()))?;

        Ok(AuthUser { id: user_id })
    }
}