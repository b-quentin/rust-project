use std::sync::Arc;
use log::trace;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, Object, SimpleObject};
use uuid::Uuid;

use crate::internal::api::admin::users::{errors::{db::AdminDbError, interface::CustomGraphQLError}, services::{auth::{ JwtTokenService, TokenService}, users::{AdminUserService, AdminUserServiceImpl, UserFilter}}};

#[derive(SimpleObject)]
pub struct UserAdmin {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Default)]
pub struct AdminUserQuery;

#[Object]
impl AdminUserQuery {
    async fn users(&self, ctx: &Context<'_>, token: String, filter: Option<UserFilter>) -> async_graphql::Result<Vec<UserAdmin>> {
        let claims = match JwtTokenService::verify_token(&token).await {
            Ok(claims) => {
                trace!("users: Token verified successfully, claims: {:?}", claims);
                claims
            },
            Err(e) => {
                return Err(e.new());
            }
        };

        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => {
                trace!("users: Database connection found for user {:?}", claims.sub);
                db
            }
            Err(e) => {
                return Err(
                    (Box::new(AdminDbError::DatabaseError(format!("{:?}", e))) as Box<dyn CustomGraphQLError>).new()
                );
            }
        };

        let _ = match AdminUserServiceImpl::get_user_permissions_from_role(db.as_ref(), claims.sub, "can_read", "/admin").await {
            Ok(_) => {
                trace!("users: User {:?} has permission to read admin home", claims.sub);
            },
            Err(_) => {
                trace!("users: User {:?} doesn't have permission from role, checking user permissions", claims.sub);
                match AdminUserServiceImpl::get_user_permissions_from_user(db.as_ref(), claims.sub, "can_read", "/admin").await {
                    Ok(_) => {
                        trace!("users: User {:?} has permission to read admin home", claims.sub);
                    },
                    Err(e) => {
                        trace!("users: User {:?} doesn't have permission to read admin home", claims.sub);
                        return Err(e.new());
                    }
                }
            }
        };

        match AdminUserServiceImpl::get_all_users(db.as_ref(), filter).await {
            Ok(users) => {
                trace!("users: Users found: {:?}", users);
                Ok(users.into_iter().map(|u| UserAdmin {
                    id: u.id,
                    username: u.username,
                    first_name: u.first_name,
                    last_name: u.last_name,
                    email: u.email,
                }).collect())
            },
            Err(e) => {
                Err(e.new())
            }
        }
    }
}
