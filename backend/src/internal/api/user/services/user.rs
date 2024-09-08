use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use uuid::Uuid;
use crate::internal::api::user::models::user;
use async_trait::async_trait;
use log::trace;

#[async_trait]
pub trait UserService {
    async fn create_user(db: &DatabaseConnection, username: String, email: String, password: String) -> Result<user::Model, sea_orm::DbErr>;
    async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<Option<user::Model>, sea_orm::DbErr>;
    async fn update_user(db: &DatabaseConnection, id: Uuid, username: Option<String>, email: Option<String>, password: Option<String>) -> Result<user::Model, sea_orm::DbErr>;
    async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<bool, sea_orm::DbErr>;
}

pub struct UserServiceImpl;

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(db: &DatabaseConnection, username: String, email: String, password: String) -> Result<user::Model, sea_orm::DbErr> {
        trace!("Creating user with username: '{}', email: '{}'", username, email);
        
        let new_user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(username.clone()),
            email: Set(email.clone()),
            password: Set(hash_password(password)),
        };

        match new_user.insert(db).await {
            Ok(user) => {
                trace!("User created successfully: {:?}", user);
                Ok(user)
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<Option<user::Model>, sea_orm::DbErr> {
        trace!("Fetching user with id: {}", id);

        match user::Entity::find_by_id(id).one(db).await {
            Ok(Some(user)) => {
                trace!("User found: {:?}", user);
                Ok(Some(user))
            },
            Ok(None) => {
                trace!("User with id {} not found", id);
                Ok(None)
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    async fn update_user(db: &DatabaseConnection, id: Uuid, username: Option<String>, email: Option<String>, password: Option<String>) -> Result<user::Model, sea_orm::DbErr> {
        trace!("Updating user with id: '{}', username: '{:?}', email: '{:?}'", id, username, email);
        
        let mut user : user::ActiveModel = match user::Entity::find_by_id(id).one(db).await {
            Ok(Some(user)) => user.into(),
            Ok(None) => {
                return Err(sea_orm::DbErr::RecordNotFound("User not found".into()));
            },
            Err(e) => {
                return Err(e);
            }
        };

        if let Some(username) = username.clone() {
            user.username = Set(username);
        }
        if let Some(email) = email.clone() {
            user.email = Set(email);
        }
        if let Some(password) = password {
            user.password = Set(hash_password(password));
        }

        match user.update(db).await {
            Ok(updated_user) => {
                trace!("User updated successfully: {:?}", updated_user);
                Ok(updated_user)
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        trace!("Deleting user with id: {}", id);

        match user::Entity::delete_by_id(id).exec(db).await {
            Ok(res) => {
                if res.rows_affected > 0 {
                    trace!("User with id {} deleted successfully", id);
                    Ok(true)
                } else {
                    trace!("User with id {} not found for deletion", id);
                    Ok(false)
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}

fn hash_password(password: String) -> String {
    // Implement password hashing here (e.g., using bcrypt)
    password // Placeholder, replace with actual hash
}
