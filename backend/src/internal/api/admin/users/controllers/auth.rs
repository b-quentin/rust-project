use std::sync::Arc;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, Error, ErrorExtensions, InputObject, Object};

use crate::internal::api::admin::users::services::auth::{AuthAdminService, AuthAdminServiceImpl};

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
        match AuthAdminServiceImpl::verify_token(&token).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e.extend()),
        }
    }
}

#[derive(Default)]
pub struct AuthAdminMutation;

#[Object]
impl AuthAdminMutation {
    async fn generate_token(&self, ctx: &Context<'_>, input: GenerateTokenInput) -> async_graphql::Result<String> {
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                return Err(Error::new("Failed to access the database connection")
                    .extend_with(|_err, extensions| {
                        extensions.set("code", "DATABASE_ACCESS_ERROR");
                        extensions.set("message", format!("An error occurred while trying to access the database connection: {:?}", e));
                    })
                );
            }
        };

        match AuthAdminServiceImpl::generate_token(db.as_ref(), input.email, input.password).await {
            Ok(token) => {
                Ok(token)
            },
            Err(e) => Err(e.extend()),
        }
    }
}
