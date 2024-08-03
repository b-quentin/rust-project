use dotenv::dotenv;

use actix_web::{web::Data, App, HttpServer};
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, InputObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sea_orm::{Database, EntityTrait, QueryFilter, Set, DatabaseConnection};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use template::internal::entities::collection;
use template::internal::entities::field;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
struct CreateCollection {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
struct CreateField {
    value_type: String,
    value_name: String,
}

#[derive(SimpleObject)]
struct Collection {
    id: Uuid,
    name: String,
}

#[derive(SimpleObject)]
struct Field {
    id: Uuid,
    collection_id: Uuid,
    value_type: String,
    value_name: String,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn collections(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Collection>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let collections = collection::Entity::find().all(db).await?;
        Ok(collections.into_iter().map(|c| Collection {
            id: c.id,
            name: c.name,
        }).collect())
    }

    async fn collection(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Option<Collection>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(c) = collection::Entity::find_by_id(id).one(db).await? {
            Ok(Some(Collection {
                id: c.id,
                name: c.name,
            }))
        } else {
            Ok(None)
        }
    }

    async fn fields(&self, ctx: &Context<'_>, collection_id: Uuid) -> async_graphql::Result<Vec<Field>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let fields = field::Entity::find()
            .filter(field::Column::CollectionId.eq(collection_id))
            .all(db)
            .await?;
        Ok(fields.into_iter().map(|f| Field {
            id: f.id,
            collection_id: f.collection_id,
            value_type: f.value_type,
            value_name: f.value_name,
        }).collect())
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_collection(&self, ctx: &Context<'_>, name: String) -> async_graphql::Result<Collection> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let new_uuid = Uuid::new_v4();
        let new_entry = collection::ActiveModel {
            id: Set(new_uuid),
            name: Set(name),
        };

        let c = new_entry.insert(db).await?;
        Ok(Collection {
            id: c.id,
            name: c.name,
        })
    }

    async fn create_field(&self, ctx: &Context<'_>, collection_id: Uuid, value_type: String, value_name: String) -> async_graphql::Result<Field> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let new_uuid = Uuid::new_v4();

        let new_entry = field::ActiveModel {
            id: Set(new_uuid),
            collection_id: Set(collection_id),
            value_type: Set(value_type),
            value_name: Set(value_name),
        };

        let f = new_entry.insert(db).await?;
        Ok(Field {
            id: f.id,
            collection_id: f.collection_id,
            value_type: f.value_type,
            value_name: f.value_name,
        })
    }

    async fn delete_collection(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let res = collection::Entity::delete_by_id(Uuid::from(id)).exec(db).await?;
        Ok(res.rows_affected > 0)
    }

    async fn delete_field(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let res = field::Entity::delete_by_id(Uuid::from(id)).exec(db).await?;
        Ok(res.rows_affected > 0)
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

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(actix_web::web::resource("/graphql").guard(actix_web::guard::Post()).to(graphql_handler))
            .service(actix_web::web::resource("/graphql").guard(actix_web::guard::Get()).to(graphql_playground))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn graphql_handler(schema: Data<Schema<QueryRoot, MutationRoot, EmptySubscription>>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> actix_web::Result<actix_web::HttpResponse> {
    let playground = async_graphql::http::GraphiQLSource::build().endpoint("/graphql").finish();
    Ok(actix_web::HttpResponse::Ok().content_type("text/html").body(playground))
}

