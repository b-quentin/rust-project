
use async_graphql::{Context, Error, Object, SimpleObject, InputObject};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::internal::api::user::services::user::{UserService, UserServiceImpl};

#[derive(SimpleObject)]
struct User {
    id: Uuid,
    username: String,
    email: String,
}

#[derive(InputObject)]
struct CreateUserInput {
    username: String,
    email: String,
    password: String,
}

#[derive(InputObject)]
struct UpdateUserInput {
    id: Uuid,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Option<User>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserServiceImpl::get_user(db, id).await.map_err(|e| Error::new(e.to_string()))?;
        Ok(user.map(|u| User {
            id: u.id,
            username: u.username,
            email: u.email,
        }))
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> async_graphql::Result<User> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserServiceImpl::create_user(db, input.username, input.email, input.password).await.map_err(|e| Error::new(e.to_string()))?;
        Ok(User {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }

    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> async_graphql::Result<User> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserServiceImpl::update_user(db, input.id, input.username, input.email, input.password).await.map_err(|e| Error::new(e.to_string()))?;
        Ok(User {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        UserServiceImpl::delete_user(db, id).await.map_err(|e| Error::new(e.to_string()))
    }
}

