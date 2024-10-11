use sea_orm::{sqlx::types::chrono::Utc, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use crate::internal::api::users::models::users;
use async_trait::async_trait;
use log::trace;

#[async_trait]
pub trait UserService {
    async fn create_user(db: &DatabaseConnection, username: String, firstname: String, lastname: String, email: String, password: String) -> Result<users::Model, sea_orm::DbErr>;
    async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<Option<users::Model>, sea_orm::DbErr>;
    async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<users::Model>, sea_orm::DbErr>;
    async fn update_user(db: &DatabaseConnection, id: Uuid, username: Option<String>, email: Option<String>, password: Option<String>) -> Result<users::Model, sea_orm::DbErr>;
    async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<bool, sea_orm::DbErr>;

    async fn find_user_by_email(db: &DatabaseConnection, email: String) -> Result<Option<users::Model>, sea_orm::DbErr>;
}

pub struct UserServiceImpl;

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(db: &DatabaseConnection, username: String, firstname: String, lastname: String, email: String, password: String) -> Result<users::Model, sea_orm::DbErr> {
        trace!("Creating user with username: '{}', email: '{}'", username, email);
        
        let new_user = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(username.clone()),
            first_name: Set(firstname.clone()),
            last_name: Set(lastname.clone()),
            email: Set(email.clone()),
            password: Set(hash_password(password)),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
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

    async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<Option<users::Model>, sea_orm::DbErr> {
        trace!("Fetching user with id: {}", id);

        match users::Entity::find_by_id(id).one(db).await {
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

    async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<users::Model>, sea_orm::DbErr> {
        trace!("Fetching all users");

        match users::Entity::find().all(db).await {
            Ok(users) => {
                trace!("Users found: {:?}", users);
                Ok(users)
             },
             Err(e) => {
                 Err(e)
             }
         }
    }

    async fn update_user(db: &DatabaseConnection, id: Uuid, username: Option<String>, email: Option<String>, password: Option<String>) -> Result<users::Model, sea_orm::DbErr> {
        trace!("Updating user with id: '{}', username: '{:?}', email: '{:?}'", id, username, email);
        
        let mut user : users::ActiveModel = match users::Entity::find_by_id(id).one(db).await {
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

        match users::Entity::delete_by_id(id).exec(db).await {
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


    async fn find_user_by_email(db: &DatabaseConnection, email: String) -> Result<Option<users::Model>, sea_orm::DbErr> {
        trace!("Searching for user with email: '{}'", email);
        
        match users::Entity::find()
            .filter(users::Column::Email.eq(email.clone())) // Ensure ColumnTrait is in scope
            .one(db)
            .await
        {
            Ok(Some(user)) => {
                trace!("User found with email: '{}'", user.email);
                Ok(Some(user))
            },
            Ok(None) => {
                trace!("No user found with email: '{}'", email);
                Ok(None)
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
