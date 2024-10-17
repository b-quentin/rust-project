use std::sync::Arc;
use log::trace;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, Error, Object, SimpleObject};
use uuid::Uuid;

use crate::internal::api::admin::users::services::users::{UserAdminService, UserAdminServiceImpl};

#[derive(SimpleObject)]
pub struct UserAdmin {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Default)]
pub struct AdminUserQuery;

#[Object]
impl AdminUserQuery {
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<UserAdmin>> {
        trace!("Fetching all users");
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                return Err(Error::new(format!("Failed to access database connection in context with error {:?}", e)));
            }
        };

        match UserAdminServiceImpl::get_all_users(db.as_ref()).await {
            Ok(users) => {
                trace!("Users found: {:?}", users);
                Ok(users.into_iter().map(|u| UserAdmin {
                    id: u.id,
                    username: u.username,
                    first_name: u.first_name,
                    last_name: u.last_name,
                    email: u.email,
                }).collect())
            },
            Err(e) => {
                Err(Error::new(format!("Failed to fetch users with error {}", e)))
            }
        }
    }
}
