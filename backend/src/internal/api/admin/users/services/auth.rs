use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Duration;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{sqlx::types::chrono::Utc, DatabaseConnection};
use async_trait::async_trait;
use log::trace;
use thiserror::Error;
use uuid::Uuid;
use crate::internal::api::admin::users::models::{admin_actions, admin_entities, admin_roles_actions_entities_assignements, admin_users, admin_users_roles};
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

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

#[async_trait]
pub trait AuthAdminService {
    async fn generate_token(db: &DatabaseConnection, username: String, password: String) -> Result<String, AuthError>;
    async fn verify_token(token: &str) -> Result<Claims, AuthError>;
    async fn get_user_permissions<'a>(db: &'a DatabaseConnection, user_id: Uuid, action: &'a str, entities: &'a str) -> Result<admin_users::Model, AuthError>;
}

pub struct AuthAdminServiceImpl;

// Modèle des claims pour le JWT
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

#[async_trait]
impl AuthAdminService for AuthAdminServiceImpl {
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

    async fn generate_token(db: &DatabaseConnection, email: String, password: String) -> Result<String, AuthError> {
        use log::trace;

        trace!("Generating token for user with username: '{}'", email);

        // Recherche de l'utilisateur dans la base de données
        let user = admin_users::Entity::find()
            .filter(admin_users::Column::Email.eq(email.clone()))
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
                    sub: user.id.clone(), 
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
            return Err(AuthError::UserNotFound(email));
        }
    }

    async fn get_user_permissions<'a>(
        db: &'a DatabaseConnection,
        user_id: Uuid,
        action: &'a str,
        entities: &'a str,
    ) -> Result<admin_users::Model, AuthError> {
        trace!("Fetching roles for user_id: {}", user_id);

        // Fetch roles associated with the user
        let user_roles = admin_users_roles::Entity::find()
            .filter(admin_users_roles::Column::AdminUserId.eq(user_id))
            .all(db)
            .await
            .map_err(AuthError::DatabaseError)?;

        if user_roles.is_empty() {
            return Err(AuthError::PermissionDenied("User has no roles assigned".to_string()));
        }

        // Fetch action ID
        let action_id = admin_actions::Entity::find()
            .filter(admin_actions::Column::Name.eq(action))
            .one(db)
            .await
            .map_err(AuthError::DatabaseError)?
            .ok_or_else(|| AuthError::NotFound("Action not found".to_string()))?
            .id;

        // Fetch entity ID
        let entity_id = admin_entities::Entity::find()
            .filter(admin_entities::Column::Name.eq(entities))
            .one(db)
            .await
            .map_err(AuthError::DatabaseError)?
            .ok_or_else(|| AuthError::NotFound("Entity not found".to_string()))?
            .id;

        // Fetch permissions associated with the user's roles
        let permissions = admin_roles_actions_entities_assignements::Entity::find()
            .filter(admin_roles_actions_entities_assignements::Column::RoleId.is_in(
                user_roles.iter().map(|role| role.role_admin_id),
            ))
            .filter(admin_roles_actions_entities_assignements::Column::PermissionId.eq(action_id))
            .filter(admin_roles_actions_entities_assignements::Column::EntityId.eq(entity_id))
            .all(db)
            .await
            .map_err(AuthError::DatabaseError)?;

        if permissions.is_empty() {
            return Err(AuthError::PermissionDenied("No permissions found for the user".to_string()));
        }

        // Fetch the user model to return
        let user = admin_users::Entity::find_by_id(user_id)
            .one(db)
            .await
            .map_err(AuthError::DatabaseError)?
            .ok_or_else(|| AuthError::NotFound("User not found".to_string()))?;

        Ok(user)
    }
}
