use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation, encode, EncodingKey, Header};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use async_trait::async_trait;
use log::trace;
use uuid::Uuid;
use std::env;
use crate::internal::api::admin::users::{
    errors::{
        action::AdminActionError, auth::AuthTokenError, db::AdminDbError, entity::AdminEntityError, interface::CustomGraphQLError, permission::AdminPermissionError, user::AdminUserAuthError
    }, 
    models::{
        admin_actions, 
        admin_entities, 
        admin_roles_actions_entities_assignements, 
        admin_users, 
        admin_users_roles
    }, services::users::{UserAdminService, UserAdminServiceImpl}
};

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

        let user = UserAdminServiceImpl::get_user_by_email(db, &email).await?;

        if password == user.password {
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
        } else {
            Err(Box::new(AdminUserAuthError::InvalidPassword))
        }
    }
}

#[async_trait]
pub trait AdminPermissionService {
    async fn get_user_permissions<'a>(db: &'a DatabaseConnection, user_id: Uuid, action: &'a str, entities: &'a str) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>>;
}

pub struct AdminRoleBasedPermissionService;

#[async_trait]
impl AdminPermissionService for AdminRoleBasedPermissionService {
    async fn get_user_permissions<'a>(
        db: &'a DatabaseConnection,
        user_id: Uuid,
        action: &'a str,
        entities: &'a str,
    ) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>> {
        trace!("Fetching roles for user_id: {}", user_id);

        let user_roles = UserAdminServiceImpl::get_user_roles(db, user_id).await?;
        if user_roles.is_empty() {
            return Err(Box::new(AdminPermissionError::PermissionDenied("User has no roles assigned".to_string())) as Box<dyn CustomGraphQLError>);
        }

        let action_id = get_action_id_by_name(db, action).await?;
        let entity_id = get_entity_id_by_name(db, entities).await?;

        let permissions = get_permissions_for_roles(db, &user_roles, action_id, entity_id).await?;
        if permissions.is_empty() {
            return Err(Box::new(AdminPermissionError::PermissionDenied("No permissions found for the user".to_string())));
        }

        UserAdminServiceImpl::get_user_by_id(db, user_id).await
    }
}

async fn get_action_id_by_name(db: &DatabaseConnection, action: &str) -> Result<Uuid, Box<dyn CustomGraphQLError>> {
    admin_actions::Entity::find()
        .filter(admin_actions::Column::Name.eq(action))
        .one(db)
        .await
        .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)?
        .ok_or_else(|| Box::new(AdminActionError::NotFound("Action not found".to_string())) as Box<dyn CustomGraphQLError>)
        .map(|action| action.id)
}

async fn get_entity_id_by_name(db: &DatabaseConnection, entity: &str) -> Result<Uuid, Box<dyn CustomGraphQLError>> {
    admin_entities::Entity::find()
        .filter(admin_entities::Column::Name.eq(entity))
        .one(db)
        .await
        .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)?
        .ok_or_else(|| Box::new(AdminEntityError::NotFound("Entity not found".to_string())) as Box<dyn CustomGraphQLError>)
        .map(|entity| entity.id)
}

async fn get_permissions_for_roles(
    db: &DatabaseConnection, 
    user_roles: &[admin_users_roles::Model], 
    action_id: Uuid, 
    entity_id: Uuid,
) -> Result<Vec<admin_roles_actions_entities_assignements::Model>, Box<dyn CustomGraphQLError>> {
    admin_roles_actions_entities_assignements::Entity::find()
        .filter(admin_roles_actions_entities_assignements::Column::RoleId.is_in(
            user_roles.iter().map(|role| role.role_admin_id),
        ))
        .filter(admin_roles_actions_entities_assignements::Column::PermissionId.eq(action_id))
        .filter(admin_roles_actions_entities_assignements::Column::EntityId.eq(entity_id))
        .all(db)
        .await
        .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)
}

