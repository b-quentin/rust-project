use async_graphql::{Context, Error, Object, SimpleObject, InputObject};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use log::trace;
use crate::internal::api::user::services::user::{UserService, UserServiceImpl};

#[derive(SimpleObject)]
struct User {
    id: Uuid,
    username: String,
    email: String,
}

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Option<User>> {
        trace!("Fetching user with id: {}", id);
        let db = ctx.data::<DatabaseConnection>()?;

        match UserServiceImpl::get_user(db, id).await {
            Ok(Some(u)) => {
                trace!("User found: {:?}", u);
                Ok(Some(User {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                }))
            },
            Ok(None) => {
                Err(Error::new(format!("User with id {} not found", id)))
            },
            Err(e) => {
                Err(Error::new(format!("Failed to fetch user with error {}", e)))
            }
        }
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> async_graphql::Result<User> {
        trace!("Creating user with username: '{}', email: '{}'", input.username, input.email);
        let db = ctx.data::<DatabaseConnection>()?;

        match UserServiceImpl::create_user(db, input.username.clone(), input.email.clone(), input.password).await {
            Ok(user) => {
                trace!("User created successfully: {:?}", user);
                Ok(User {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                })
            },
            Err(e) => {
                Err(Error::new(format!("Failed to create user with username '{}', email '{}': {}", input.username, input.email, e)))
            }
        }
    }

    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> async_graphql::Result<User> {
        trace!("Updating user with id: '{}', username: '{:?}', email: '{:?}'", input.id, input.username, input.email);
        let db = ctx.data::<DatabaseConnection>()?;

        match UserServiceImpl::update_user(db, input.id, input.username.clone(), input.email.clone(), input.password).await {
            Ok(user) => {
                trace!("User updated successfully: {:?}", user);
                Ok(User {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                })
            },
            Err(e) => {
                Err(Error::new(format!("Failed to update user with id '{}', username '{:?}', email '{:?}': {}", input.id, input.username, input.email, e)))
            }
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        trace!("Deleting user with id: {}", id);
        let db = ctx.data::<DatabaseConnection>()?;

        match UserServiceImpl::delete_user(db, id).await {
            Ok(result) => {
                trace!("User with id {} deleted successfully", id);
                Ok(result)
            },
            Err(e) => {
                Err(Error::new(format!("Failed to delete user with id '{}': {}", id, e)))
            }
        }
    }
}

