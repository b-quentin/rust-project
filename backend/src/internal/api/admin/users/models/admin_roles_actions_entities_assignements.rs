use sea_orm::entity::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "admin_roles_permissions_entities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub permission_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub entity_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::admin_roles::Entity", from = "Column::RoleId", to = "super::admin_roles::Column::Id")]
    Role,
    #[sea_orm(belongs_to = "super::admin_actions::Entity", from = "Column::PermissionId", to = "super::admin_actions::Column::Id")]
    Action,
    #[sea_orm(belongs_to = "super::admin_entities::Entity", from = "Column::EntityId", to = "super::admin_entities::Column::Id")]
    Entity,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
