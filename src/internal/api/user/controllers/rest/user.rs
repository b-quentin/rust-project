use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::internal::api::user::services::user::{UserService, UserServiceImpl};

#[derive(Serialize, Deserialize)]
pub struct CreateUserInput {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserInput {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

pub async fn get_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    match UserServiceImpl::get_user(db.get_ref(), id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_user(db: web::Data<DatabaseConnection>, user_data: web::Json<CreateUserInput>) -> impl Responder {
    match UserServiceImpl::create_user(db.get_ref(), user_data.username.clone(), user_data.email.clone(), user_data.password.clone()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>, user_data: web::Json<UpdateUserInput>) -> impl Responder {
    match UserServiceImpl::update_user(db.get_ref(), id.into_inner(), user_data.username.clone(), user_data.email.clone(), user_data.password.clone()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    match UserServiceImpl::delete_user(db.get_ref(), id.into_inner()).await {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

