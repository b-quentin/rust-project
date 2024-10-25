use async_graphql::InputObject;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use async_trait::async_trait;
use log::trace;
use uuid::Uuid;
use crate::internal::api::admin::users::{errors::{db::AdminDbError, interface::CustomGraphQLError, permission::AdminPermissionError, user::AdminUserAuthError}, models::{admin_users, admin_users_roles}, services::{actions::{AdminActionService, AdminActionServiceImpl}, entities::{AdminEntitiesService, AdminEntitiesServiceImpl}, permissions::{AdminPermissionService, AdminPermissionServiceImpl}}};

#[derive(InputObject)]
pub struct UserFilter {
    pub id : Option<Uuid>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[async_trait]
pub trait AdminUserService {
    async fn get_all_users(db: &DatabaseConnection, filter: Option<UserFilter>) -> Result<Vec<admin_users::Model>, Box<dyn CustomGraphQLError>>;
    async fn get_user_by_id(db: &DatabaseConnection, user_id: Uuid) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>>;
    async fn get_user_by_email(db: &DatabaseConnection, email: &str) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>>;
    async fn get_user_roles(db: &DatabaseConnection, user_id: Uuid) -> Result<Vec<admin_users_roles::Model>, Box<dyn CustomGraphQLError>>;
    async fn get_user_permissions_from_role<'a>(
        db: &'a DatabaseConnection,
        user_id: Uuid,
        action: &'a str,
        entities: &'a str,
    ) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>>;

    async fn get_user_permissions_from_user<'a>(
        db: &'a DatabaseConnection,
        user_id: Uuid,
        action: &'a str,
        entities: &'a str,
    ) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>>;
}

pub struct AdminUserServiceImpl;


#[async_trait]
impl AdminUserService for AdminUserServiceImpl {
    async fn get_all_users(db: &DatabaseConnection, filter: Option<UserFilter>) -> Result<Vec<admin_users::Model>, Box<dyn CustomGraphQLError>> {
        trace!("Fetching all users");

        let mut query = admin_users::Entity::find();

        if let Some(filter) = filter {
            if let Some(id) = filter.id {
                query = query.filter(admin_users::Column::Id.eq(id));
            }
            if let Some(email) = filter.email {
                query = query.filter(admin_users::Column::Email.eq(email));
            }
            if let Some(username) = filter.username {
                query = query.filter(admin_users::Column::Username.eq(username));
            }
            if let Some(first_name) = filter.first_name {
                query = query.filter(admin_users::Column::FirstName.eq(first_name));
            }
            if let Some(last_name) = filter.last_name {
                query = query.filter(admin_users::Column::LastName.eq(last_name));
            }
        }

         match query.all(db).await {
            Ok(users) => {
                trace!("Users found: {:?}", users);
                Ok(users)
            },
            Err(e) => Err(Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>),
        }
    }

    async fn get_user_by_id(db: &DatabaseConnection, user_id: Uuid) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>> {
        admin_users::Entity::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)?
            .ok_or_else(|| Box::new(AdminUserAuthError::UserNotFound("User not found".to_string())) as Box<dyn CustomGraphQLError>)
    }

    async fn get_user_by_email(db: &DatabaseConnection, email: &str) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>> {
        admin_users::Entity::find()
            .filter(admin_users::Column::Email.eq(email))
            .one(db)
            .await
            .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)?
            .ok_or_else(|| Box::new(AdminUserAuthError::UserNotFound(email.to_string())) as Box<dyn CustomGraphQLError>)
    }

    async fn get_user_roles(db: &DatabaseConnection, user_id: Uuid) -> Result<Vec<admin_users_roles::Model>, Box<dyn CustomGraphQLError>> {
        admin_users_roles::Entity::find()
            .filter(admin_users_roles::Column::AdminUserId.eq(user_id))
            .all(db)
            .await
            .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)
    }

    async fn get_user_permissions_from_role<'a>(
        db: &'a DatabaseConnection,
        user_id: Uuid,
        action: &'a str,
        entities: &'a str,
    ) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>> {
        trace!("Fetching roles for user_id: {}", user_id);

        let user_roles: Result<Vec<admin_users_roles::Model>, Box<dyn CustomGraphQLError>> = match AdminUserServiceImpl::get_user_roles(db, user_id).await {
            Ok(user_roles) => {
                if user_roles.is_empty() {
                    Err(Box::new(AdminPermissionError::PermissionDenied("User has no roles assigned".to_string())) as Box<dyn CustomGraphQLError>)
                } else {
                trace!("get_user_permissions: user_roles: {:?}", user_roles);
                    Ok(user_roles)
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        let action_id: Result<Uuid, Box<dyn CustomGraphQLError>> = match AdminActionServiceImpl::get_action_id_by_name(db, action).await {
            Ok(action_id) => {
                trace!("get_user_permissions: action_id: {}", action_id);
                Ok(action_id)
            }
            Err(e) => {
                return Err(e);
            }
        };

        let entity_id: Result<Uuid, Box<dyn CustomGraphQLError>> = match AdminEntitiesServiceImpl::get_entity_id_by_name(db, entities).await {
            Ok(entity_id) => {
                trace!("get_user_permissions: entity_id: {}", entity_id);
                Ok(entity_id)
            }
            Err(e) => {
                return Err(e);
            }
        };

        match AdminPermissionServiceImpl::get_permissions_for_roles(db, &user_roles?, action_id?, entity_id?).await {
            Ok(permissions) => {
                if permissions.is_empty() {
                    return Err(Box::new(AdminPermissionError::PermissionDenied("No permissions found for the user".to_string())));
                } else {
                    trace!("get_user_permissions: permissions: {:?}", permissions);
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        match AdminUserServiceImpl::get_user_by_id(db, user_id).await {
            Ok(user) => {
                trace!("get_user_permissions: user: {:?}", user);
                Ok(user)
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    async fn get_user_permissions_from_user<'a>(
        db: &'a DatabaseConnection,
        user_id: Uuid,
        action: &'a str,
        entities: &'a str,
    ) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>> {
        trace!("Fetching roles for user_id: {}", user_id);


        let action_id: Result<Uuid, Box<dyn CustomGraphQLError>> = match AdminActionServiceImpl::get_action_id_by_name(db, action).await {
            Ok(action_id) => {
                trace!("get_user_permissions: action_id: {}", action_id);
                Ok(action_id)
            }
            Err(e) => {
                return Err(e);
            }
        };

        let entity_id: Result<Uuid, Box<dyn CustomGraphQLError>> = match AdminEntitiesServiceImpl::get_entity_id_by_name(db, entities).await {
            Ok(entity_id) => {
                trace!("get_user_permissions: entity_id: {}", entity_id);
                Ok(entity_id)
            }
            Err(e) => {
                return Err(e);
            }
        };

        match AdminPermissionServiceImpl::get_permissions_for_user(db, user_id, action_id?, entity_id?).await {
            Ok(permissions) => {
                if permissions.is_empty() {
                    return Err(Box::new(AdminPermissionError::PermissionDenied("No permissions found for the user".to_string())));
                } else {
                    trace!("get_user_permissions: permissions: {:?}", permissions);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        match AdminUserServiceImpl::get_user_by_id(db, user_id).await {
            Ok(user) => {
                trace!("get_user_permissions: user: {:?}", user);
                Ok(user)
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
