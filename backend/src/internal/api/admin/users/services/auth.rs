use std::time::{SystemTime, UNIX_EPOCH};
use async_graphql::{Error, ErrorExtensions};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation, encode, EncodingKey, Header};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use async_trait::async_trait;
use log::trace;
use thiserror::Error;
use uuid::Uuid;
use crate::internal::api::admin::users::models::{admin_actions, admin_entities, admin_roles_actions_entities_assignements, admin_users, admin_users_roles};
use std::env;

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

    #[error("Token expired")]
    TokenExpired,
}

impl AuthError {
    pub fn extend(self) -> Error {
        Error::new(self.to_string()).extend_with(|_err, extensions| {
            match self {
                AuthError::DatabaseError(_) => {
                    extensions.set("code", "DATABASE_ERROR");
                    extensions.set("message", "An error occurred while accessing the database.");
                }
                AuthError::JwtError(_) => {
                    extensions.set("code", "JWT_ERROR");
                    extensions.set("message", "Failed to decode or validate the JWT.");
                }
                AuthError::InvalidPassword => {
                    extensions.set("code", "INVALID_PASSWORD");
                    extensions.set("message", "Password mismatch.");
                }
                AuthError::UserNotFound(ref user) => {
                    extensions.set("code", "USER_NOT_FOUND");
                    extensions.set("message", format!("User not found: {}", user));
                }
                AuthError::UnexpectedError(ref msg) => {
                    extensions.set("code", "UNEXPECTED_ERROR");
                    extensions.set("message", msg.clone());
                }
                AuthError::PermissionDenied(ref msg) => {
                    extensions.set("code", "PERMISSION_DENIED");
                    extensions.set("message", msg.clone());
                }
                AuthError::NotFound(ref resource) => {
                    extensions.set("code", "RESOURCE_NOT_FOUND");
                    extensions.set("message", format!("Resource not found: {}", resource));
                }
                AuthError::TokenExpired => {
                    extensions.set("code", "TOKEN_EXPIRED");
                    extensions.set("message", "Token expired.");
                }
            }
        })
    }
}

#[async_trait]
pub trait AuthAdminService {
    async fn generate_token(db: &DatabaseConnection, email: String, password: String) -> Result<String, AuthError>;
    async fn verify_token(token: &str) -> Result<Claims, AuthError>;
    async fn get_user_permissions<'a>(db: &'a DatabaseConnection, user_id: Uuid, action: &'a str, entities: &'a str) -> Result<admin_users::Model, AuthError>;
}

pub struct AuthAdminServiceImpl;

// Model for JWT claims
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

impl Claims {
    fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;
        self.exp < now
    }
}

#[async_trait]
impl AuthAdminService for AuthAdminServiceImpl {
    async fn verify_token(token: &str) -> Result<Claims, AuthError> {
        trace!("Verifying token: {}", token);

        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "votre_secret".to_string());

        let token_data = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()), 
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(e) => return Err(AuthError::JwtError(e)),
        };

        if token_data.claims.is_expired() {
            return Err(AuthError::TokenExpired);
        }

        Ok(token_data.claims)
    }

    async fn generate_token(db: &DatabaseConnection, email: String, password: String) -> Result<String, AuthError> {
        trace!("Generating token for user with email: '{}'", email);

        let user = match admin_users::Entity::find()
            .filter(admin_users::Column::Email.eq(email.clone()))
            .one(db)
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => return Err(AuthError::UserNotFound(email)),
            Err(e) => return Err(AuthError::DatabaseError(e)),
        };

        if password == user.password {
            let expiration = match Utc::now().checked_add_signed(Duration::seconds(3600)) {
                Some(expiration) => expiration.timestamp(),
                None => return Err(AuthError::UnexpectedError("Failed to create expiration timestamp".to_string())),
            };

            let claims = Claims { 
                sub: user.id.clone(), 
                exp: expiration as usize 
            };

            let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "votre_secret".to_string());

            let token = match encode(
                &Header::default(), 
                &claims, 
                &EncodingKey::from_secret(secret.as_ref())
            ) {
                Ok(token) => token,
                Err(e) => return Err(AuthError::JwtError(e)),
            };

            Ok(token)
        } else {
            Err(AuthError::InvalidPassword)
        }
    }

    async fn get_user_permissions<'a>(
        db: &'a DatabaseConnection,
        user_id: Uuid,
        action: &'a str,
        entities: &'a str,
    ) -> Result<admin_users::Model, AuthError> {
        trace!("Fetching roles for user_id: {}", user_id);

        let user_roles = match admin_users_roles::Entity::find()
            .filter(admin_users_roles::Column::AdminUserId.eq(user_id))
            .all(db)
            .await 
        {
            Ok(roles) => roles,
            Err(e) => return Err(AuthError::DatabaseError(e)),
        };

        if user_roles.is_empty() {
            return Err(AuthError::PermissionDenied("User has no roles assigned".to_string()));
        }

        let action_id = match admin_actions::Entity::find()
            .filter(admin_actions::Column::Name.eq(action))
            .one(db)
            .await 
        {
            Ok(Some(action)) => action.id,
            Ok(None) => return Err(AuthError::NotFound("Action not found".to_string())),
            Err(e) => return Err(AuthError::DatabaseError(e)),
        };

        let entity_id = match admin_entities::Entity::find()
            .filter(admin_entities::Column::Name.eq(entities))
            .one(db)
            .await 
        {
            Ok(Some(entity)) => entity.id,
            Ok(None) => return Err(AuthError::NotFound("Entity not found".to_string())),
            Err(e) => return Err(AuthError::DatabaseError(e)),
        };

        let permissions = match admin_roles_actions_entities_assignements::Entity::find()
            .filter(admin_roles_actions_entities_assignements::Column::RoleId.is_in(
                user_roles.iter().map(|role| role.role_admin_id),
            ))
            .filter(admin_roles_actions_entities_assignements::Column::PermissionId.eq(action_id))
            .filter(admin_roles_actions_entities_assignements::Column::EntityId.eq(entity_id))
            .all(db)
            .await 
        {
            Ok(perms) => perms,
            Err(e) => return Err(AuthError::DatabaseError(e)),
        };

        if permissions.is_empty() {
            return Err(AuthError::PermissionDenied("No permissions found for the user".to_string()));
        }

        let user = match admin_users::Entity::find_by_id(user_id)
            .one(db)
            .await 
        {
            Ok(Some(user)) => user,
            Ok(None) => return Err(AuthError::NotFound("User not found".to_string())),
            Err(e) => return Err(AuthError::DatabaseError(e)),
        };

        Ok(user)
    }
}

