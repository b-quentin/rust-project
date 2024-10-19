use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::internal::api::admin::users::{errors::{action::AdminActionError, db::AdminDbError, interface::CustomGraphQLError}, models::admin_actions};

#[async_trait]
pub trait AdminActionService {
    async fn get_action_id_by_name(db: &DatabaseConnection, action: &str) -> Result<Uuid, Box<dyn CustomGraphQLError>>;
}

pub struct AdminActionServiceImpl;

#[async_trait]
impl AdminActionService for AdminActionServiceImpl {
    async fn get_action_id_by_name(db: &DatabaseConnection, action: &str) -> Result<Uuid, Box<dyn CustomGraphQLError>> {
        admin_actions::Entity::find()
            .filter(admin_actions::Column::Name.eq(action))
            .one(db)
            .await
            .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)?
            .ok_or_else(|| Box::new(AdminActionError::NotFound("Action not found".to_string())) as Box<dyn CustomGraphQLError>)
            .map(|action| action.id)
    }
}
