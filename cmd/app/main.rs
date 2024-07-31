use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{error::ErrorNotFound, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Hello World")
}

#[actix_web::get("/greet/{id}")]
async fn greet_with_id(user_id: web::Path<u32>) -> impl Responder {
    format!("Hello {user_id} !")
}

#[derive(Serialize,Deserialize)]
struct User {
    name: String,
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    name: String,
}

#[actix_web::post("/users/create")]
async fn create_user(
    user_data: web::Json<User>,
    db: web::Data<UserDb>
) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();

    db.insert(new_id, user_data.into_inner());

    HttpResponse::Created().json(CreateUserResponse {
        id: new_id,
        name,
    })
}

#[actix_web::get("/users/{id}")]
async fn get_user(
    user_id: web::Path<u32>,
    db: web::Data<UserDb>
) -> impl Responder {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();

    match db.get(&user_id) {
        Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        None => Err(ErrorNotFound("User not found"))
    }
}

#[derive(Serialize,Deserialize,Clone)]
struct Field {
    id: Option<Uuid>,
    value_type: String,
    value_name: String,
}

#[derive(Serialize,Deserialize)]
struct Collection {
    id: Option<Uuid>,
    name: String,
    fields: Vec<Field>
}

type CollectionDb = Arc<Mutex<HashMap<Uuid, Collection>>>;

#[derive(Serialize)]
struct CreateCollectionResponse {
    id: Uuid,
    name: String,
    field: Vec<Field>,
}

#[actix_web::post("/collections/create")]
async fn create_collection(
    collection_data: web::Json<Collection>,
    db: web::Data<CollectionDb>
) -> impl Responder {
    let mut db = db.lock().unwrap();

    let new_id = Uuid::new_v4();
    let name = collection_data.name.clone();

    let mut collection = collection_data.into_inner();
    collection.id = Some(new_id);
    for field in &mut collection.fields {
        field.id = Some(Uuid::new_v4());
    }

    let field = collection.fields.clone();
    db.insert(new_id, collection);

    HttpResponse::Created().json(CreateCollectionResponse {
        id: new_id,
        name,
        field
    })
}

#[actix_web::get("/collections/{id}")]
async fn get_collection(
    collection_id: web::Path<Uuid>,
    db: web::Data<CollectionDb>
) -> impl Responder {
    let collection_id = collection_id.into_inner();
    let db = db.lock().unwrap();

    match db.get(&collection_id) {
        Some(collection_data) => Ok(HttpResponse::Ok().json(collection_data)),
        None => Err(ErrorNotFound("Collection not found"))
            
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let address = "127.0.0.1";
    println!("Starting server on {address}:{port}");

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));
    let collection_db: CollectionDb = Arc::new(Mutex::new(HashMap::<Uuid, Collection>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        let collection_data = web::Data::new(collection_db.clone());

        App::new()
            .app_data(app_data)
            .app_data(collection_data)
            .service(greet)
            .service(greet_with_id)
            .service(create_user)
            .service(get_user)
            .service(create_collection)
            .service(get_collection)
    })
    .bind((address, port))?
    .workers(2)
    .run()
    .await
}
