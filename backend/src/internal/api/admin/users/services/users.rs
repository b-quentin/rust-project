use sea_orm::{DatabaseConnection, EntityTrait};
use async_trait::async_trait;
use log::trace;
use crate::internal::api::admin::users::{errors::{db::AdminDbError, interface::CustomGraphQLError}, models::admin_users};

#[async_trait]
pub trait UserAdminService {
    async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<admin_users::Model>, Box<dyn CustomGraphQLError>>;
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
}
