use std::sync::Arc;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, InputObject, Object};

use crate::internal::api::admin::users::{errors::db::DbError, services::auth::{AuthAdminService, AuthAdminServiceImpl}};

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
                return Err(DbError::DatabaseError(format!("{:?}", e)).extend());
            }
        };

        match AuthAdminServiceImpl::generate_token(db.as_ref(), input.email, input.password).await {
            Ok(token) => {
                Ok(token)
            },
            Err(e) => { 
                return Err(e.extend()); 
            },
        }
    }
}
