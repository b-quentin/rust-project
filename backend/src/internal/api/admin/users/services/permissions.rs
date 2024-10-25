use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::internal::api::admin::users::{errors::{db::AdminDbError, interface::CustomGraphQLError}, models::{admin_roles_actions_entities_assignements, admin_users_actions_entities_assignements, admin_users_roles}};

#[async_trait]
pub trait AdminPermissionService {
    async fn get_permissions_for_roles(
        db: &DatabaseConnection,
        user_roles: &[admin_users_roles::Model],
        action_id: Uuid,
        entity_id: Uuid,
    ) -> Result<Vec<admin_roles_actions_entities_assignements::Model>, Box<dyn CustomGraphQLError>>;

    async fn get_permissions_for_user(
        db: &DatabaseConnection,
        user_id: Uuid,
        action_id: Uuid,
        entity_id: Uuid,
    ) -> Result<Vec<admin_users_actions_entities_assignements::Model>, Box<dyn CustomGraphQLError>>;
}

pub struct AdminPermissionServiceImpl;

#[async_trait]
impl AdminPermissionService for AdminPermissionServiceImpl {
    async fn get_permissions_for_roles(
        db: &DatabaseConnection,
        user_roles: &[admin_users_roles::Model],
        action_id: Uuid,
        entity_id: Uuid,
    ) -> Result<Vec<admin_roles_actions_entities_assignements::Model>, Box<dyn CustomGraphQLError>> {

        let role_ids: Vec<_> = user_roles.iter().map(|role| role.role_admin_id).collect();

        admin_roles_actions_entities_assignements::Entity::find()
            .filter(admin_roles_actions_entities_assignements::Column::RoleId.is_in(role_ids))
            .filter(admin_roles_actions_entities_assignements::Column::PermissionId.eq(action_id))
            .filter(admin_roles_actions_entities_assignements::Column::EntityId.eq(entity_id))
            .all(db)
            .await
            .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)
    }
    async fn get_permissions_for_user(
        db: &DatabaseConnection,
        user_id: Uuid,
        action_id: Uuid,
        entity_id: Uuid,
    ) -> Result<Vec<admin_users_actions_entities_assignements::Model>, Box<dyn CustomGraphQLError>> {

        admin_users_actions_entities_assignements::Entity::find()
            .filter(admin_users_actions_entities_assignements::Column::UserId.eq(user_id))
            .filter(admin_users_actions_entities_assignements::Column::PermissionId.eq(action_id))
            .filter(admin_users_actions_entities_assignements::Column::EntityId.eq(entity_id))
            .all(db)
            .await
            .map_err(|e| Box::new(AdminDbError::DatabaseError(e.to_string())) as Box<dyn CustomGraphQLError>)
    }
}
