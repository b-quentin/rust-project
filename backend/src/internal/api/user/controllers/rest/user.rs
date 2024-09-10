use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use log::{error, trace};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::internal::api::user::services::user::{UserService, UserServiceImpl};

#[derive(Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub async fn get_user(db: web::Data<Arc<DatabaseConnection>>, id: web::Path<Uuid>) -> impl Responder {
    let user_id = id.into_inner();
    trace!("Attempting to fetch user with id: {}", user_id);

    match UserServiceImpl::get_user(db.get_ref().as_ref(), user_id).await {
        Ok(Some(user)) => {
            trace!("User found: {:?}", user);
            HttpResponse::Ok().json(user)
        },
        Ok(None) => {
            trace!("User with id {} not found", user_id);
            HttpResponse::NotFound().finish()
        },
        Err(e) => {
            error!("Failed to get user with id {}: {:?}", user_id, e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn create_user(db: web::Data<Arc<DatabaseConnection>>, user_data: web::Json<CreateUserInput>) -> impl Responder {
    let user_data = user_data.into_inner();
    trace!("Attempting to create user with username: '{}', email: '{}'", user_data.username, user_data.email);

    match UserServiceImpl::create_user(db.get_ref().as_ref(), user_data.username.clone(), user_data.email.clone(), user_data.password).await {
        Ok(user) => {
            trace!("User created successfully: {:?}", user);
            HttpResponse::Created().json(user)
        },
        Err(e) => {
            error!("Failed to create user with username '{}', email '{}': {:?}", user_data.username, user_data.email, e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn update_user(db: web::Data<Arc<DatabaseConnection>>, id: web::Path<Uuid>, user_data: web::Json<UpdateUserInput>) -> impl Responder {
    let user_id = id.into_inner();
    let user_data = user_data.into_inner();
    trace!("Attempting to update user with id: '{}', username: '{:?}', email: '{:?}'", user_id, user_data.username, user_data.email);

    match UserServiceImpl::update_user(db.get_ref().as_ref(), user_id, user_data.username.clone(), user_data.email.clone(), user_data.password).await {
        Ok(user) => {
            trace!("User updated successfully: {:?}", user);
            HttpResponse::Ok().json(user)
        },
        Err(e) => {
            error!("Failed to update user with id '{}', username '{:?}', email '{:?}': {:?}", user_id, user_data.username, user_data.email, e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn delete_user(db: web::Data<Arc<DatabaseConnection>>, id: web::Path<Uuid>) -> impl Responder {
    let user_id = id.into_inner();
    trace!("Attempting to delete user with id: {}", user_id);

    match UserServiceImpl::delete_user(db.get_ref().as_ref(), user_id).await {
        Ok(true) => {
            trace!("User with id {} deleted successfully", user_id);
            HttpResponse::Ok().finish()
        },
        Ok(false) => {
            trace!("User with id {} not found for deletion", user_id);
            HttpResponse::NotFound().finish()
        },
        Err(e) => {
            error!("Failed to delete user with id '{}': {:?}", user_id, e);
            HttpResponse::InternalServerError().finish()
        },
    }
}
