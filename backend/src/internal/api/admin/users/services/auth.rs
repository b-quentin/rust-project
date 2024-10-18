use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation, encode, EncodingKey, Header};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use async_trait::async_trait;
use log::trace;
use uuid::Uuid;
use crate::internal::api::admin::users::{errors::auth::AdminAuthError, models::{admin_actions, admin_entities, admin_roles_actions_entities_assignements, admin_users, admin_users_roles}};
use std::env;



#[async_trait]
pub trait AuthAdminService {
    async fn generate_token(db: &DatabaseConnection, email: String, password: String) -> Result<String, AdminAuthError>;
    async fn verify_token(token: &str) -> Result<Claims, AdminAuthError>;
    async fn get_user_permissions<'a>(db: &'a DatabaseConnection, user_id: Uuid, action: &'a str, entities: &'a str) -> Result<admin_users::Model, AdminAuthError>;
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
    async fn verify_token(token: &str) -> Result<Claims, AdminAuthError> {
        trace!("Verifying token: {}", token);

        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "votre_secret".to_string());

        let token_data = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()), 
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(e) => return Err(AdminAuthError::JwtError(e)),
        };

        if token_data.claims.is_expired() {
            return Err(AdminAuthError::TokenExpired);
        }

        Ok(token_data.claims)
    }

    async fn generate_token(db: &DatabaseConnection, email: String, password: String) -> Result<String, AdminAuthError> {
        trace!("Generating token for user with email: '{}'", email);

        let user = match admin_users::Entity::find()
            .filter(admin_users::Column::Email.eq(email.clone()))
            .one(db)
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => return Err(AdminAuthError::UserNotFound(email)),
            Err(e) => return Err(AdminAuthError::DatabaseError(e)),
        };

        if password == user.password {
            let expiration = match Utc::now().checked_add_signed(Duration::seconds(3600)) {
                Some(expiration) => expiration.timestamp(),
                None => return Err(AdminAuthError::UnexpectedError("Failed to create expiration timestamp".to_string())),
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
                Err(e) => return Err(AdminAuthError::JwtError(e)),
            };

            Ok(token)
        } else {
            Err(AdminAuthError::InvalidPassword)
        }
    }

    async fn get_user_permissions<'a>(
        db: &'a DatabaseConnection,
        user_id: Uuid,
        action: &'a str,
        entities: &'a str,
    ) -> Result<admin_users::Model, AdminAuthError> {
        trace!("Fetching roles for user_id: {}", user_id);

        let user_roles = match admin_users_roles::Entity::find()
            .filter(admin_users_roles::Column::AdminUserId.eq(user_id))
            .all(db)
            .await 
        {
            Ok(roles) => roles,
            Err(e) => return Err(AdminAuthError::DatabaseError(e)),
        };

        if user_roles.is_empty() {
            return Err(AdminAuthError::PermissionDenied("User has no roles assigned".to_string()));
        }

        let action_id = match admin_actions::Entity::find()
            .filter(admin_actions::Column::Name.eq(action))
            .one(db)
            .await 
        {
            Ok(Some(action)) => action.id,
            Ok(None) => return Err(AdminAuthError::NotFound("Action not found".to_string())),
            Err(e) => return Err(AdminAuthError::DatabaseError(e)),
        };

        let entity_id = match admin_entities::Entity::find()
            .filter(admin_entities::Column::Name.eq(entities))
            .one(db)
            .await 
        {
            Ok(Some(entity)) => entity.id,
            Ok(None) => return Err(AdminAuthError::NotFound("Entity not found".to_string())),
            Err(e) => return Err(AdminAuthError::DatabaseError(e)),
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
            Err(e) => return Err(AdminAuthError::DatabaseError(e)),
        };

        if permissions.is_empty() {
            return Err(AdminAuthError::PermissionDenied("No permissions found for the user".to_string()));
        }

        let user = match admin_users::Entity::find_by_id(user_id)
            .one(db)
            .await 
        {
            Ok(Some(user)) => user,
            Ok(None) => return Err(AdminAuthError::NotFound("User not found".to_string())),
            Err(e) => return Err(AdminAuthError::DatabaseError(e)),
        };

        Ok(user)
    }
}

