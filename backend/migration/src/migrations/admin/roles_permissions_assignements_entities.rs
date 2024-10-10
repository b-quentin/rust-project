use sea_orm_migration::prelude::*;

use super::{admin_entities::AdminEntities, admin_roles::AdminRoles, permissions::AdminActions};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminRolesPermissionsEntities {
    Table,
    RoleId,
    PermissionId,
    EntityId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminRolesPermissionsEntities::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminRolesPermissionsEntities::RoleId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminRolesPermissionsEntities::PermissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminRolesPermissionsEntities::EntityId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminRolesPermissionsEntities::Table, AdminRolesPermissionsEntities::RoleId)
                            .to(AdminRoles::Table, AdminRoles::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminRolesPermissionsEntities::Table, AdminRolesPermissionsEntities::PermissionId)
                            .to(AdminActions::Table, AdminActions::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminRolesPermissionsEntities::Table, AdminRolesPermissionsEntities::EntityId)
                            .to(AdminEntities::Table, AdminEntities::Id),
                    )
                    .primary_key(
                        Index::create()
                            .col(AdminRolesPermissionsEntities::RoleId)
                            .col(AdminRolesPermissionsEntities::PermissionId)
                            .col(AdminRolesPermissionsEntities::EntityId),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AdminRolesPermissionsEntities::Table).to_owned()).await?;
        Ok(())
    }
}
