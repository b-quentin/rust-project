use sea_orm::entity::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "admin_roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>, // Champ optionnel
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::admin_users_roles::Entity", from = "Column::Id", to = "super::admin_users_roles::Column::RoleAdminId")]
    AdminUsers,
    #[sea_orm(belongs_to = "super::admin_roles_actions_entities_assignements::Entity", from = "Column::Id", to = "super::admin_roles_actions_entities_assignements::Column::RoleId")]
    Role,
}


#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

