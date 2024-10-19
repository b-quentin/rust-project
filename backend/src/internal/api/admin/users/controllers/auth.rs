use std::sync::Arc;
use log::trace;
use sea_orm::DatabaseConnection;
use async_graphql::{Context, InputObject, Object};

use crate::internal::api::admin::users::{errors::{db::AdminDbError, interface::CustomGraphQLError}, services::auth::{JwtTokenService, TokenService}};

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
