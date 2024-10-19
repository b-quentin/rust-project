use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::internal::api::admin::users::{errors::{db::AdminDbError, entity::AdminEntityError, interface::CustomGraphQLError}, models::admin_entities};


#[async_trait]
pub trait AdminEntitiesService {
    async fn get_entity_id_by_name(db: &DatabaseConnection, entity: &str) -> Result<Uuid, Box<dyn CustomGraphQLError>>;
}

pub struct AdminEntitiesServiceImpl;

#[async_trait]
impl AdminEntitiesService for AdminEntitiesServiceImpl {
    async fn get_entity_id_by_name(db: &DatabaseConnection, entity: &str) -> Result<Uuid, Box<dyn CustomGraphQLError>> {
        admin_entities::Entity::find()
            .filter(admin_entities::Column::Name.eq(entity))
            .one(db)
            .await
            .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)?
            .ok_or_else(|| Box::new(AdminEntityError::NotFound("Entity not found".to_string())) as Box<dyn CustomGraphQLError>)
            .map(|entity| entity.id)
    }
}
