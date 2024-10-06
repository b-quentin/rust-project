use actix_cors::Cors;
use actix_web::http;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use async_graphql::{Schema, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use log::{debug, error, info};
use sea_orm::Database;
use std::env;
use std::sync::Arc;
use template::internal::api::user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_port = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_user, database_password, database_url, database_port, database_name
    );

    info!("Connecting to database...");
    debug!("With this connection string: {}", connection_string);

    let db = match Database::connect(&connection_string).await {
        Ok(db) => {
            info!("Connected to the database successfully");
            Arc::new(db)
        }
        Err(e) => {
            error!("Failed to connect to the database: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed"));
        }
    };

    let schema = Schema::build(
        user::controllers::graphql::QueryRoot,
        user::controllers::graphql::MutationRoot,
        EmptySubscription,
    )
    .data(db.clone())
    .finish();

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    info!("Server is running on http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(db.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|origin, _req_head| {
                        let allowed_origins = env::var("ALLOWED_ORIGINS")
                            .unwrap_or_default()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<String>>();

                        allowed_origins.contains(&origin.to_str().unwrap_or_default().to_string())
                    })
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(actix_web::web::resource("/graphql").guard(actix_web::guard::Post()).to(graphql_handler))
            .service(actix_web::web::resource("/graphql").guard(actix_web::guard::Get()).to(graphql_playground))
    })
    .bind(&bind_address)?
    .run()
    .await
}

async fn graphql_handler(schema: Data<Schema<user::controllers::graphql::QueryRoot, user::controllers::graphql::MutationRoot, EmptySubscription>>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> actix_web::Result<actix_web::HttpResponse> {
    let playground = async_graphql::http::GraphiQLSource::build().endpoint("/graphql").finish();
    Ok(actix_web::HttpResponse::Ok().content_type("text/html").body(playground))
}
