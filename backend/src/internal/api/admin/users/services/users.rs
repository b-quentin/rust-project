use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use async_trait::async_trait;
use log::trace;
use uuid::Uuid;
use crate::internal::api::admin::users::{errors::{db::AdminDbError, interface::CustomGraphQLError, user::AdminUserAuthError}, models::{admin_users, admin_users_roles}};

#[async_trait]
pub trait UserAdminService {
    async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<admin_users::Model>, Box<dyn CustomGraphQLError>>;
    async fn get_user_by_id(db: &DatabaseConnection, user_id: Uuid) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>>;
    async fn get_user_by_email(db: &DatabaseConnection, email: &str) -> Result<admin_users::Model, Box<dyn CustomGraphQLError>>;
    async fn get_user_roles(db: &DatabaseConnection, user_id: Uuid) -> Result<Vec<admin_users_roles::Model>, Box<dyn CustomGraphQLError>>;
}

pub struct UserAdminServiceImpl;


#[async_trait]
impl UserAdminService for UserAdminServiceImpl {
    async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<admin_users::Model>, Box<dyn CustomGraphQLError>> {
        trace!("Fetching all users");

        match admin_users::Entity::find().all(db).await {
            Ok(users) => {
                trace!("Users found: {:?}", users);
                Ok(users)
             },
             Err(e) => {
                 Err(Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)
             }
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
}
