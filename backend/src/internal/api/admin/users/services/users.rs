use sea_orm::{DatabaseConnection, EntityTrait};
use async_trait::async_trait;
use log::trace;
use crate::internal::api::admin::users::models::admin_users;

#[async_trait]
pub trait UserAdminService {
    async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<admin_users::Model>, sea_orm::DbErr>;
}

pub struct UserAdminServiceImpl;


#[async_trait]
impl UserAdminService for UserAdminServiceImpl {
    async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<admin_users::Model>, sea_orm::DbErr> {
        trace!("Fetching all users");

        match admin_users::Entity::find().all(db).await {
            Ok(users) => {
                trace!("Users found: {:?}", users);
                Ok(users)
             },
             Err(e) => {
                 Err(e)
             }
         }
    }
}

