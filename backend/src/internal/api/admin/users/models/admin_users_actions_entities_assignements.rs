use sea_orm::entity::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "admin_users_permissions_entities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub permission_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub entity_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::admin_users::Entity", from = "Column::UserId", to = "super::admin_users::Column::Id")]
    Role,
    #[sea_orm(belongs_to = "super::admin_actions::Entity", from = "Column::PermissionId", to = "super::admin_actions::Column::Id")]
    Action,
    #[sea_orm(belongs_to = "super::admin_entities::Entity", from = "Column::EntityId", to = "super::admin_entities::Column::Id")]
    Entity,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
