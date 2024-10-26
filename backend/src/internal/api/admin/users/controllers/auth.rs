use std::sync::Arc;
use log::trace;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, InputObject, Object};

use crate::internal::api::admin::users::{errors::{db::AdminDbError, interface::CustomGraphQLError}, services::{auth::{JwtTokenService, TokenService}, users::{AdminUserService, AdminUserServiceImpl}}};

#[derive(InputObject)]
pub struct GenerateTokenInput {
    pub email: String,
    pub password: String,
}

#[derive(Default)]
pub struct AuthAdminQuery;

#[Object]
impl AuthAdminQuery {
    async fn verify_token(&self, token: String) -> async_graphql::Result<bool> {
        match JwtTokenService::verify_token(&token).await {
            Ok(_) => {
                trace!("Verify token: Token verified successfully {:?}", token);
                Ok(true)
            },
            Err(e) => {
                return Err(e.new());
            }
        }
    }
    async fn get_access_page(&self,ctx: &Context<'_>, token: String, page: String) -> async_graphql::Result<bool> {
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

        let has_permission = match AdminUserServiceImpl::get_user_permissions_from_role(db.as_ref(), claims.sub, "can_read", &page).await {
            Ok(_) => {
                trace!("users: User {:?} has permission to read admin home", claims.sub);
                Ok(true)
            },
            Err(_) => {
                trace!("users: User {:?} doesn't have permission from role, checking user permissions", claims.sub);
                match AdminUserServiceImpl::get_user_permissions_from_user(db.as_ref(), claims.sub, "can_read", &page).await {
                    Ok(_) => {
                        trace!("users: User {:?} has permission to read admin home", claims.sub);
                        Ok(true)
                    },
                    Err(e) => {
                        trace!("users: User {:?} doesn't have permission to read admin home", claims.sub);
                        return Err(e.new());
                    }
                }
            }
        };

        has_permission
    }
}

#[derive(Default)]
pub struct AuthAdminMutation;

#[Object]
impl AuthAdminMutation {
    async fn generate_token(&self, ctx: &Context<'_>, input: GenerateTokenInput) -> async_graphql::Result<String> {
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => {
                trace!("Generate token: Database connection found");
                db
            },
            Err(e) => {
                return Err(
                    (Box::new(AdminDbError::DatabaseError(format!("{:?}", e))) as Box<dyn CustomGraphQLError>).new()
                );
            }
        };

        match JwtTokenService::generate_token(db.as_ref(), input.email, input.password).await {
            Ok(token) => {
                trace!("Generate token: Token generated successfully {:?}", token);
                Ok(token)
            },
            Err(e) => { 
                return Err(e.new()); 
            },
        }
    }
}
