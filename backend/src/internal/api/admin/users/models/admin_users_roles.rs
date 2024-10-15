use sea_orm::entity::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "admin_users_admin_roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub admin_user_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_admin_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::admin_users::Entity", from = "Column::AdminUserId", to = "super::admin_users::Column::Id")]
    AdminUsers,
    #[sea_orm(belongs_to = "super::admin_roles::Entity", from = "Column::RoleAdminId", to = "super::admin_roles::Column::Id")]
    AdminRoles,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
