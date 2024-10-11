use std::sync::Arc;
use async_graphql::{Context, Error, Object, SimpleObject, InputObject};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use log::{error, trace};
use crate::internal::api::users::services::users::{UserService, UserServiceImpl};

#[derive(SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
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
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                return Err(Error::new(format!("Failed to access database connection in context with error {:?}", e)));
            }
        };

        match UserServiceImpl::get_user(db.as_ref(), id).await {
            Ok(Some(u)) => {
                trace!("User found: {:?}", u);
                Ok(Some(User {
                    id: u.id,
                    username: u.username,
                    first_name: u.first_name,
                    last_name: u.last_name,
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
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        trace!("Fetching all users");
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                return Err(Error::new(format!("Failed to access database connection in context with error {:?}", e)));
            }
        };

        match UserServiceImpl::get_all_users(db.as_ref()).await {
            Ok(users) => {
                trace!("Users found: {:?}", users);
                Ok(users.into_iter().map(|u| User {
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

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> async_graphql::Result<User> {
        trace!("Creating user with username: '{}', email: '{}'", input.username, input.email);
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                return Err(Error::new(format!("Failed to access database connection in context with error {:?}", e)));
            }
        };

        // Check if the email already exists in the database
        //match UserServiceImpl::find_user_by_email(db.as_ref(), input.email.clone()).await {
        //    Ok(Some(_)) => {
        //        return Err(Error::new(format!("Email '{}' is already in use.", input.email)));
        //    },
        //    Ok(None) => {
        //        // Email does not exist, proceed with user creation
        //    },
        //    Err(e) => {
        //        return Err(Error::new(format!("Failed to check email uniqueness: {}", e)));
        //    }
        //}

        match UserServiceImpl::create_user(db.as_ref(), input.username.clone(), input.first_name.clone(), input.last_name.clone(), input.email.clone(), input.password).await {
            Ok(user) => {
                trace!("User created successfully: {:?}", user);
                Ok(User {
                    id: user.id,
                    username: user.username,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                })
            },
            Err(e) => {
                error!("Failed to create usser with username '{}', email '{}', first_name '{}', last_name '{}', error: {}", input.username, input.email, input.first_name, input.last_name, e);
                Err(Error::new(format!("Failed to create user with username '{}', email '{}', first_name '{}', last_name '{}', error: {}", input.username, input.email, input.first_name, input.last_name, e)))
            }
        }
    }

    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> async_graphql::Result<User> {
        trace!("Updating user with id: '{}', username: '{:?}', email: '{:?}'", input.id, input.username, input.email);
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                error!("Failed to access database connection in context with error {:?}", e);
                return Err(Error::new(format!("Failed to access database connection in context with error {:?}", e)));
            }
        };

        match UserServiceImpl::update_user(db.as_ref(), input.id, input.username.clone(), input.email.clone(), input.password).await {
            Ok(user) => {
                trace!("User updated successfully: {:?}", user);
                Ok(User {
                    id: user.id,
                    username: user.username,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                })
            },
            Err(e) => {
                error!("Failed to update user with id '{}', username '{:?}', email '{:?}': {}", input.id, input.username, input.email, e);
                Err(Error::new(format!("Failed to update user with id '{}', username '{:?}', email '{:?}': {}", input.id, input.username, input.email, e)))
            }
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        trace!("Deleting user with id: {}", id);
        let db = match ctx.data::<Arc<DatabaseConnection>>() {
            Ok(db) => db,
            Err(e) => {
                error!("Failed to access database connection in context with error {:?}", e);
                return Err(Error::new(format!("Failed to access database connection in context with error {:?}", e)));
            }
        };

        match UserServiceImpl::delete_user(db.as_ref(), id).await {
            Ok(result) => {
                trace!("User with id {} deleted successfully", id);
                Ok(result)
            },
            Err(e) => {
                error!("Failed to delete user with id '{}': {}", id, e);
                Err(Error::new(format!("Failed to delete user with id '{}': {}", id, e)))
            }
        }
    }
}

