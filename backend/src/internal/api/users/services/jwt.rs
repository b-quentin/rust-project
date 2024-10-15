use async_graphql::Context;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug)]
pub enum JwtError {
    EnvVarNotSet,
    TimeError(std::time::SystemTimeError),
    EncodingError(jsonwebtoken::errors::Error),
    DecodingError(jsonwebtoken::errors::Error),
}

fn get_jwt_secret() -> Result<String, JwtError> {
    dotenv().ok();
    match env::var("JWT_SECRET") {
        Ok(secret) => Ok(secret),
        Err(_) => Err(JwtError::EnvVarNotSet),
    }
}

pub fn generate_jwt(user_id: &str) -> Result<String, JwtError> {
    let secret = get_jwt_secret()?;

    // Token expiration time logic
    let expiration = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() + 3600, // 1 hour
        Err(e) => return Err(JwtError::TimeError(e)),
    };

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(token) => Ok(token),
        Err(e) => Err(JwtError::EncodingError(e)),
    }
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, JwtError> {
    let secret = get_jwt_secret()?;

    let token_data = match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
        Ok(data) => data,
        Err(e) => return Err(JwtError::DecodingError(e)),
    };

    // Check expiration
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as usize,
        Err(e) => return Err(JwtError::TimeError(e)),
    };

    match token_data.claims.exp < now {
        true => Err(JwtError::DecodingError(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::ExpiredSignature,
        ))),
        false => Ok(token_data),
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    PermissionDenied,
}

//pub fn check_user_role(ctx: &Context<'_>, required_role: &str) -> Result<(), AuthError> {
//    let token = match ctx.data::<String>() {
//        Ok(data) => data,
//        Err(_) => return Err(AuthError::MissingToken),
//    };
//
//    match validate_jwt(token) {
//        Ok(token_data) => {
//            match token_data.claims.role == required_role {
//                true => Ok(()),
//                false => Err(AuthError::PermissionDenied),
//            }
//        }
//        Err(_) => Err(AuthError::InvalidToken),
//    }
//}
