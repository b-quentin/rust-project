use std::sync::Arc;
use log::trace;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, Object, SimpleObject};
use uuid::Uuid;

use crate::internal::api::admin::users::{errors::db::DbError, services::{auth::{AuthAdminService, AuthAdminServiceImpl}, users::{UserAdminService, UserAdminServiceImpl}}};

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
                return Err(e.extend());
            }
        };

        let _ = match AuthAdminServiceImpl::get_user_permissions(ctx.data::<Arc<DatabaseConnection>>().unwrap().as_ref(), claims.sub, "can_read", "Pages::AdminHome").await {
            Ok(_) => {},
            Err(e) => {
                return Err(e.extend());
            }
        };

        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                return Err(DbError::DatabaseError(format!("{:?}", e)).extend());
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
                Err(e.extend())
            }
        }
    }
}
