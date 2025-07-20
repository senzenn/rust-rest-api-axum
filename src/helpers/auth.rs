use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use uuid::Uuid;
use crate::model::model::Claims;
use anyhow::Result;
use tracing::info;

const JWT_SECRET: &str = "your-secret-key-change-in-production";

pub struct AuthHelper;

impl AuthHelper {
    pub fn hash_password(password: &str) -> Result<String> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let is_valid = verify(password, hash)?;
        Ok(is_valid)
    }

    pub fn generate_token(user_id: Uuid) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: Utc::now().timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET.as_ref()),
        )?;

        info!("Generated JWT token for user: {}", user_id);
        Ok(token)
    }

    pub fn validate_token(token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub fn extract_user_id_from_token(token: &str) -> Result<Uuid> {
        let claims = Self::validate_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub)?;
        Ok(user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "test_password";
        let hash = AuthHelper::hash_password(password).unwrap();
        assert!(AuthHelper::verify_password(password, &hash).unwrap());
        assert!(!AuthHelper::verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_jwt_token() {
        let user_id = Uuid::new_v4();
        let token = AuthHelper::generate_token(user_id).unwrap();
        let claims = AuthHelper::validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
    }
} 