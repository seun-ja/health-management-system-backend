pub mod appointment;
pub mod health;
pub mod login;
pub mod signup;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, routes::login::LoggedUser};

pub fn verify_password(
    encrypted_password: &str,
    password: &str,
    hmac_secret: &str,
    id: &str,
    email: &str,
) -> Result<LoggedUser, ApiError> {
    let expected_password_hash = PasswordHash::new(encrypted_password)
        .map_err(|e| ApiError::InvalidPasswordHash(e.to_string()))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &expected_password_hash)
        .map_err(|_| ApiError::InvalidCredentials)?;

    Ok(LoggedUser {
        token: create_jwt(id, email, hmac_secret)?,
    })
}

pub fn encrypt_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), salt.as_salt())
        .map_err(ApiError::FailedHashingPassword)?;

    Ok(hashed_password.to_string())
}

/// JWT token creator
/// Creates a JWT token with a 1-day expiration time
/// and the provided user ID and email.
pub fn create_jwt(uid: &str, email: &str, hmac_secret: &str) -> Result<String, ApiError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .map(|dt| dt.timestamp())
        .ok_or(ApiError::WrongTimeStamp)?;

    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize,
        email: email.to_owned(),
    };

    let header = Header::new(Algorithm::HS256);
    jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(hmac_secret.as_bytes()),
    )
    .map_err(|e| ApiError::InvalidJWTCredentials(e.to_string()))
}

/// JWT claims
#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    exp: usize,
}
