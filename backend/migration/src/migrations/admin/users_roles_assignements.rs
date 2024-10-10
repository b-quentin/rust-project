use sea_orm_migration::prelude::*;

use super::{admin_users::AdminUsers, admin_roles::AdminRoles};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminUsersAdminRoles {
    Table,
    AdminUserId,
    RoleAdminId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminUsersAdminRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUsersAdminRoles::AdminUserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminUsersAdminRoles::RoleAdminId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminUsersAdminRoles::Table, AdminUsersAdminRoles::AdminUserId)
                            .to(AdminUsers::Table, AdminUsers::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminUsersAdminRoles::Table, AdminUsersAdminRoles::RoleAdminId)
                            .to(AdminRoles::Table, AdminRoles::Id),
                    )
                    .primary_key(
                        Index::create()
                            .col(AdminUsersAdminRoles::AdminUserId)
                            .col(AdminUsersAdminRoles::RoleAdminId),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AdminUsersAdminRoles::Table).to_owned()).await?;
        Ok(())
    }
}
