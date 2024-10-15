use std::sync::Arc;
use log::trace;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, Error, Object, SimpleObject};
use uuid::Uuid;

use crate::internal::api::admin::users::services::{auth::{AuthAdminService, AuthAdminServiceImpl}, users::{UserAdminService, UserAdminServiceImpl}};

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
    async fn users(&self, ctx: &Context<'_>, token: String) -> async_graphql::Result<Vec<UserAdmin>> {
        let claims = match AuthAdminServiceImpl::verify_token(&token).await {
            Ok(claims) => {
                trace!("Token verified successfully, claims: {:?}", claims);
                claims
            },
            Err(e) => {
                return Err(Error::new(format!("Failed to verify token with error {}", e)));
            }
        };

        let _ = match AuthAdminServiceImpl::get_user_permissions(ctx.data::<Arc<DatabaseConnection>>().unwrap().as_ref(), claims.sub, "can_read", "Pages::AdminHome").await {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::new(format!("Failed to get user permissions with error {}", e)));
            }
        };

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
