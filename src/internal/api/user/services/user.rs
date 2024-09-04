use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use uuid::Uuid;
use crate::internal::api::user::models::user;
use async_trait::async_trait;

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
        let new_user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(username),
            email: Set(email),
            password: Set(hash_password(password)),
        };
        new_user.insert(db).await
    }

    async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }

    async fn update_user(db: &DatabaseConnection, id: Uuid, username: Option<String>, email: Option<String>, password: Option<String>) -> Result<user::Model, sea_orm::DbErr> {
        let mut user: user::ActiveModel = user::Entity::find_by_id(id).one(db).await?.unwrap().into();
        if let Some(username) = username {
            user.username = Set(username);
        }
        if let Some(email) = email {
            user.email = Set(email);
        }
        if let Some(password) = password {
            user.password = Set(hash_password(password));
        }
        user.update(db).await
    }

    async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        let res = user::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected > 0)
    }
}

fn hash_password(password: String) -> String {
    // Implement password hashing here (e.g., using bcrypt)
    password // Placeholder, replace with actual hash
}
