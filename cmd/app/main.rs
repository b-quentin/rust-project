use dotenv::dotenv;
use std::env;
use actix_web::{App, HttpServer, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path};
use sea_orm::{Database, EntityTrait, Set, ActiveModelTrait, DatabaseConnection};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use template::internal::entities::collection;
use template::internal::entities::field;

#[derive(Debug, Serialize, Deserialize)]
struct CreateCollection {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateField {
    value_type: String,
    value_name: String,
}



#[actix_web::get("/collections")]
async fn get_collections(db: Data<DatabaseConnection>) -> impl Responder {
    match collection::Entity::find().find_with_related(field::Entity).all(db.get_ref()).await {
        Ok(entries) => {
            HttpResponse::Ok().json(entries)
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[actix_web::post("/collections")]
async fn create_collection(db: Data<DatabaseConnection>, item: Json<CreateCollection>) -> impl Responder {
    let new_uuid = Uuid::new_v4();
    let new_entry = collection::ActiveModel {
        id: Set(new_uuid),
        name: Set(item.name.clone()),
    };

    match new_entry.insert(db.get_ref()).await {
        Ok(_) => {
            match collection::Entity::find_by_id(new_uuid).one(db.get_ref()).await {
                Ok(Some(entry)) => {
                    HttpResponse::Ok().json(entry)
                },
                Ok(None) => {
                    HttpResponse::NotFound().finish()
                },
                Err(_) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::get("/collections/{id}")]
async fn get_collection(db: Data<DatabaseConnection>, id: Path<Uuid>) -> impl Responder {
    match collection::Entity::find_by_id(*id).one(db.get_ref()).await {
        Ok(Some(entry)) => {
            HttpResponse::Ok().json(entry)
        },
        Ok(None) => {
            HttpResponse::NotFound().finish()
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::get("/collections/{id}/fields")]
async fn get_fields(db: Data<DatabaseConnection>, id: Path<Uuid>) -> impl Responder {
    match field::Entity::find().filter(field::Column::CollectionId.eq(*id)).all(db.get_ref()).await {
        Ok(entries) => {
            HttpResponse::Ok().json(entries)
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::post("/collections/{id}/fields")]
async fn create_field(db: Data<DatabaseConnection>, id: Path<Uuid>, item: Json<CreateField>) -> impl Responder {
    let new_uuid = Uuid::new_v4(); 

    let new_entry = field::ActiveModel {
        id: Set(new_uuid),
        collection_id: Set(*id),
        value_type: Set(item.value_type.clone()),
        value_name: Set(item.value_name.clone()),
    };
    
    match new_entry.insert(db.get_ref()).await {
        Ok(_) => {
            match field::Entity::find_by_id(new_uuid).one(db.get_ref()).await {
                Ok(Some(entry)) => {
                    HttpResponse::Ok().json(entry)
                },
                Ok(None) => {
                    HttpResponse::NotFound().finish()
                },
                Err(_) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_port = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_user, database_password, database_url, database_port, database_name
    );

    let db = Database::connect(&connection_string).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .service(get_collections)
            .service(create_collection)
            .service(get_fields)
            .service(create_field)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

