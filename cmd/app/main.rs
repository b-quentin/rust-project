use dotenv::dotenv;
use actix_web::{web::Data, App, HttpServer};
use async_graphql::{Context, EmptySubscription, Enum, InputObject, Object, Schema, SimpleObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sea_orm::{Database, EntityTrait, QueryFilter, Set, DatabaseConnection, QueryOrder};
use sea_orm::prelude::*;
use std::env;
use template::internal::entities::collection;
use template::internal::entities::field;
use uuid::Uuid;

#[derive(SimpleObject)]
struct Collection {
    id: Uuid,
    name: String,
    fields: Vec<Field>,
}

#[derive(SimpleObject)]
struct Field {
    id: Uuid,
    collection_id: Uuid,
    value_type: String,
    name: String,
}

#[derive(InputObject)]
struct CollectionFilter {
    id: Option<Uuid>,
    name: Option<String>,
    field_filter: Option<Box<FieldFilter>>,
}

#[derive(InputObject)]
struct FieldFilter {
    id: Option<Uuid>,
    name: Option<String>,
    collection_id: Option<Uuid>,
    value_type: Option<String>,
}

#[derive(InputObject)]
struct CollectionOrderBy {
    name: Option<OrderDirection>,
}

#[derive(InputObject)]
struct FieldOrderBy {
    name: Option<OrderDirection>,
    value_type: Option<OrderDirection>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
enum OrderDirection {
    Asc,
    Desc,
}

struct QueryRoot;

#[Object]
impl QueryRoot {


    async fn collections(
        &self,
        ctx: &Context<'_>,
        collection_filter: Option<CollectionFilter>,
        field_filter: Option<FieldFilter>,
        order_by: Option<CollectionOrderBy>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> async_graphql::Result<Vec<Collection>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut query = collection::Entity::find();

        if let Some(filter) = &collection_filter {
            if let Some(id) = filter.id {
                query = query.filter(collection::Column::Id.eq(id));
            }
            if let Some(name) = &filter.name {
                query = query.filter(collection::Column::Name.contains(name));
            }
        }

        if let Some(order_by) = order_by {
            if let Some(direction) = order_by.name {
                query = match direction {
                    OrderDirection::Asc => query.order_by_asc(collection::Column::Name),
                    OrderDirection::Desc => query.order_by_desc(collection::Column::Name),
                };
            }
        }

        let paginator = query.paginate(db, limit.unwrap_or(10).try_into().unwrap());
        let collections = paginator.fetch_page(offset.unwrap_or(0).try_into().unwrap()).await?;

        let mut result = Vec::new();
        for collection in collections {
            let mut field_query = field::Entity::find().filter(field::Column::CollectionId.eq(collection.id));

            if let Some(filter) = &field_filter {
                if let Some(id) = filter.id {
                    field_query = field_query.filter(field::Column::Id.eq(id));
                }
                if let Some(value_type) = &filter.value_type {
                    field_query = field_query.filter(field::Column::ValueType.contains(value_type));
                }
                if let Some(name) = &filter.name {
                    field_query = field_query.filter(field::Column::Name.contains(name));
                }
            }

            let fields = field_query.all(db).await?;

            if fields.is_empty() {
                continue;
            }

            result.push(Collection {
                id: collection.id,
                name: collection.name,
                fields: fields.into_iter().map(|f| Field {
                    id: f.id,
                    collection_id: f.collection_id,
                    value_type: f.value_type,
                    name: f.name,
                }).collect(),
            });
        }

        Ok(result)
    }


    async fn fields(
        &self,
        ctx: &Context<'_>,
        filter: Option<FieldFilter>,
        order_by: Option<FieldOrderBy>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> async_graphql::Result<Vec<Field>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut query = field::Entity::find();

        if let Some(filter) = filter {
            if let Some(id) = filter.id {
                query = query.filter(field::Column::Id.eq(id));
            }
            if let Some(collection_id) = filter.collection_id {
                query = query.filter(field::Column::CollectionId.eq(collection_id));
            }
            if let Some(value_type) = filter.value_type {
                query = query.filter(field::Column::ValueType.contains(&value_type));
            }
            if let Some(name) = filter.name {
                query = query.filter(field::Column::ValueType.contains(&name));
            }
        }

        if let Some(order_by) = order_by {
            if let Some(direction) = order_by.name {
                query = match direction {
                    OrderDirection::Asc => query.order_by_asc(field::Column::Name),
                    OrderDirection::Desc => query.order_by_desc(field::Column::Name),
                };
            }
            if let Some(direction) = order_by.value_type {
                query = match direction {
                    OrderDirection::Asc => query.order_by_asc(field::Column::ValueType),
                    OrderDirection::Desc => query.order_by_desc(field::Column::ValueType),
                };
            }
        }

        let paginator = query.paginate(db, limit.unwrap_or(10).try_into().unwrap());
        let fields = paginator.fetch_page(offset.unwrap_or(0).try_into().unwrap()).await?;

        Ok(fields.into_iter().map(|f| Field {
            id: f.id,
            collection_id: f.collection_id,
            value_type: f.value_type,
            name: f.name,
        }).collect())
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_collection(&self, ctx: &Context<'_>, name: String) -> async_graphql::Result<Collection> {
        let db = ctx.data::<DatabaseConnection>()?;
        let new_uuid = Uuid::new_v4();
        let new_entry = collection::ActiveModel {
            id: Set(new_uuid),
            name: Set(name),
        };

        let c = new_entry.insert(db).await?;
        Ok(Collection {
            id: c.id,
            name: c.name,
            fields: Vec::new(),
        })
    }

    async fn create_field(&self, ctx: &Context<'_>, collection_id: Uuid, value_type: String, name: String) -> async_graphql::Result<Field> {
        let db = ctx.data::<DatabaseConnection>()?;
        let new_uuid = Uuid::new_v4();

        let new_entry = field::ActiveModel {
            id: Set(new_uuid),
            collection_id: Set(collection_id),
            value_type: Set(value_type),
            name: Set(name),
        };

        let f = new_entry.insert(db).await?;
        Ok(Field {
            id: f.id,
            collection_id: f.collection_id,
            value_type: f.value_type,
            name: f.name,
        })
    }

    async fn update_collection(&self, ctx: &Context<'_>, id: Uuid, name: String) -> async_graphql::Result<Collection> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut collection: collection::ActiveModel = collection::Entity::find_by_id(id).one(db).await?.unwrap().into();
        collection.name = Set(name);
        let c = collection.update(db).await?;
        Ok(Collection {
            id: c.id,
            name: c.name,
            fields: Vec::new(),
        })
    }

    async fn update_field(&self, ctx: &Context<'_>, id: Uuid, collection_id: Uuid, value_type: String, name: String) -> async_graphql::Result<Field> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut field: field::ActiveModel = field::Entity::find_by_id(id).one(db).await?.unwrap().into();
        field.collection_id = Set(collection_id);
        field.value_type = Set(value_type);
        field.name = Set(name);
        let f = field.update(db).await?;
        Ok(Field {
            id: f.id,
            collection_id: f.collection_id,
            value_type: f.value_type,
            name: f.name,
        })
    }

    async fn delete_collection(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let res = collection::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected > 0)
    }

    async fn delete_field(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let res = field::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected > 0)
    }

    async fn bulk_delete_collections(&self, ctx: &Context<'_>, ids: Vec<Uuid>) -> async_graphql::Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let res = collection::Entity::delete_many()
            .filter(collection::Column::Id.is_in(ids))
            .exec(db)
            .await?;
        Ok(res.rows_affected > 0)
    }

    async fn bulk_delete_fields(&self, ctx: &Context<'_>, ids: Vec<Uuid>) -> async_graphql::Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let res = field::Entity::delete_many()
            .filter(field::Column::Id.is_in(ids))
            .exec(db)
            .await?;
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
