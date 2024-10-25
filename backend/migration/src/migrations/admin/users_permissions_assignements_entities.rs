use sea_orm_migration::prelude::*;

use super::{admin_entities::AdminEntities, admin_users::AdminUsers, permissions::AdminActions};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminUsersPermissionsEntities {
    Table,
    UserId,
    PermissionId,
    EntityId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminUsersPermissionsEntities::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUsersPermissionsEntities::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminUsersPermissionsEntities::PermissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminUsersPermissionsEntities::EntityId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminUsersPermissionsEntities::Table, AdminUsersPermissionsEntities::UserId)
                            .to(AdminUsers::Table, AdminUsers::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminUsersPermissionsEntities::Table, AdminUsersPermissionsEntities::PermissionId)
                            .to(AdminActions::Table, AdminActions::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminUsersPermissionsEntities::Table, AdminUsersPermissionsEntities::EntityId)
                            .to(AdminEntities::Table, AdminEntities::Id),
                    )
                    .primary_key(
                        Index::create()
                            .col(AdminUsersPermissionsEntities::UserId)
                            .col(AdminUsersPermissionsEntities::PermissionId)
                            .col(AdminUsersPermissionsEntities::EntityId),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AdminUsersPermissionsEntities::Table).to_owned()).await?;
        Ok(())
    }
}
