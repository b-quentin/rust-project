use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use async_graphql::{Schema, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use log::info;
use sea_orm::Database;
use std::env;
use template::internal::api::user;
use template::internal::api::user::routes::init_user_routes;

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

    info!("Connecting to database with: {}", connection_string);

    let db = Database::connect(&connection_string).await.unwrap();

    let schema = Schema::build(
        user::controllers::graphql::QueryRoot,
        user::controllers::graphql::MutationRoot,
        EmptySubscription,
    )
    .data(db.clone())
    .finish();

    info!("Server is running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(db.clone()))
            .service(actix_web::web::resource("/graphql").guard(actix_web::guard::Post()).to(graphql_handler))
            .service(actix_web::web::resource("/graphql").guard(actix_web::guard::Get()).to(graphql_playground))
            .configure(init_user_routes)
    })
    .bind("127.0.0.1:8080")?
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

