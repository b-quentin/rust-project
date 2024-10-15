use std::sync::Arc;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, Error, InputObject, Object};

use crate::internal::api::admin::users::services::users::{UserAdminService, UserAdminServiceImpl};

#[derive(InputObject)]
pub struct GenerateTokenInput {
    pub username: String,
    pub password: String,
}

#[derive(Default)]
pub struct AdminUserQueryRoot;

#[Object]
impl AdminUserQueryRoot {
    async fn verify_token(&self, token: String) -> async_graphql::Result<bool> {
        match UserAdminServiceImpl::verify_token(&token).await {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::new(format!("Failed to verify token with error {}", e)))
        }
    }
}

#[derive(Default)]
pub struct AdminUserMutationRoot;

#[Object]
impl AdminUserMutationRoot {
    async fn generate_token(&self, ctx: &Context<'_>, input: GenerateTokenInput) -> async_graphql::Result<String> {
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                return Err(Error::new(format!("Failed to access database connection in context with error {:?}", e)));
            }
        };

        match UserAdminServiceImpl::generate_token(db.as_ref(), input.username, input.password).await {
            Ok(token) => {
                Ok(token)
            },
            Err(e) => {
                Err(Error::new(format!("Failed to generate token with error {}", e)))
            }
        }
    }
}
