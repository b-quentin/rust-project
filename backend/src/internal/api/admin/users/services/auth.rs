use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation, encode, EncodingKey, Header};
use sea_orm::DatabaseConnection;
use async_trait::async_trait;
use log::trace;
use uuid::Uuid;
use std::env;
use crate::internal::api::admin::users::{
    errors::{
        auth::AuthTokenError, interface::CustomGraphQLError, user::AdminUserAuthError
    }, 
    services::users::{
        AdminUserService,
        AdminUserServiceImpl
    },
};
use bcrypt::verify;

#[async_trait]
pub trait TokenService {
    async fn generate_token(db: &DatabaseConnection, email: String, password: String) -> Result<String, Box<dyn CustomGraphQLError>>;
    async fn verify_token(token: &str) -> Result<Claims, Box<dyn CustomGraphQLError>>;
}

pub struct JwtTokenService;

// Model for JWT claims
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

impl Claims {
    fn is_expired(&self) -> bool {
        let now = current_time_as_secs();
        self.exp < now
    }
}

fn current_time_as_secs() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "votre_secret".to_string())
}

#[async_trait]
impl TokenService for JwtTokenService {
    async fn verify_token(token: &str) -> Result<Claims, Box<dyn CustomGraphQLError>> {
        trace!("Verifying token: {}", token);

        let secret = get_jwt_secret();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()), 
            &Validation::default(),
        ).map_err(|e| Box::new(AuthTokenError::JwtError(e)) as Box<dyn CustomGraphQLError>)?;

        if token_data.claims.is_expired() {
            return Err(Box::new(AuthTokenError::TokenExpired) as Box<dyn CustomGraphQLError>);
        }

        Ok(token_data.claims)
    }

    async fn generate_token(db: &DatabaseConnection, email: String, password: String) -> Result<String, Box<dyn CustomGraphQLError>> {
        trace!("Generating token for user with email: '{}'", email);

        let user = AdminUserServiceImpl::get_user_by_email(db, &email).await?;

        match verify(&password, &user.password) {
            Ok(_) => {
                let expiration = Utc::now()
                    .checked_add_signed(Duration::seconds(3600))
                    .ok_or_else(|| Box::new(AdminUserAuthError::UnexpectedError("Failed to create expiration timestamp".to_string())) as Box<dyn CustomGraphQLError>)?
                    .timestamp();

                let claims = Claims { 
                    sub: user.id.clone(), 
                    exp: expiration as usize 
                };

                let secret = get_jwt_secret();
                let token = encode(
                    &Header::default(), 
                    &claims, 
                    &EncodingKey::from_secret(secret.as_ref())
                ).map_err(|e| Box::new(AuthTokenError::JwtError(e)) as Box<dyn CustomGraphQLError>)?;

                Ok(token)
            },
            Err(_) => {
                trace!("Invalid password for user with email: '{}'", email);
                trace!("Password: '{}'", password);
                trace!("User: {:?}", user);

                Err(Box::new(AdminUserAuthError::InvalidPassword))
            }
        }
    }
}
