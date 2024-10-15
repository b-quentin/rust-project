use sea_orm::entity::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "admin_entities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::admin_roles_actions_entities_assignements::Entity", from = "Column::Id", to = "super::admin_roles_actions_entities_assignements::Column::EntityId")]
    Entity,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

