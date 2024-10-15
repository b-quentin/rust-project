use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Duration;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{sqlx::types::chrono::Utc, DatabaseConnection};
use async_trait::async_trait;
use log::trace;
use thiserror::Error;
use crate::internal::api::admin::users::models::admin_users;
use sea_orm::EntityTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Password mismatch")]
    InvalidPassword,

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

#[async_trait]
pub trait UserAdminService {
    async fn generate_token(db: &DatabaseConnection, username: String, password: String) -> Result<String, AuthError>;
    async fn verify_token(token: &str) -> Result<Claims, AuthError>;
}

pub struct UserAdminServiceImpl;

// Modèle des claims pour le JWT
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    sub: String,   // Sujet (username)
    exp: usize,    // Expiration du token en timestamp UNIX
}

#[async_trait]
impl UserAdminService for UserAdminServiceImpl {
    // Function to verify the JWT token
    async fn verify_token(token: &str) -> Result<Claims, AuthError> {
        trace!("Verifying token: {}", token);

        // Decode the token using the same secret used to sign it
        let token_data: TokenData<Claims> = decode::<Claims>(
            token,
            &DecodingKey::from_secret("votre_secret".as_ref()), // Same secret used to encode
            &Validation::default(), // Default validation (can be customized)
        ).map_err(AuthError::JwtError)?;

        // Check token expiration
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as usize;

        if token_data.claims.exp < now {
            return Err(AuthError::UnexpectedError("Token expired".to_string()));
        }

        // Return the claims if token is valid
        Ok(token_data.claims)
    }

    async fn generate_token(db: &DatabaseConnection, username: String, password: String) -> Result<String, AuthError> {
        use log::trace;

        trace!("Generating token for user with username: '{}'", username);

        // Recherche de l'utilisateur dans la base de données
        let user = admin_users::Entity::find()
            .filter(admin_users::Column::Username.eq(username.clone()))
            .one(db)
            .await?;

        if let Some(user) = user {
            if password == user.password {
                // Génération d'un token JWT
                let expiration = Utc::now()
                    .checked_add_signed(Duration::seconds(3600))  // Token expire dans 1 heure
                    .expect("failed to create expiration timestamp")
                    .timestamp();

                let claims = Claims { 
                    sub: user.username.clone(), 
                    exp: expiration as usize 
                };

                // Encode le token avec une clé secrète (à sécuriser)
                let token = encode(
                    &Header::default(), 
                    &claims, 
                    &EncodingKey::from_secret("votre_secret".as_ref())  // Remplacez "votre_secret" par une clé plus sécurisée
                )?;

                Ok(token)
            } else {
                return Err(AuthError::InvalidPassword);
            }
        } else {
            return Err(AuthError::UserNotFound(username));
        }
    }
}

