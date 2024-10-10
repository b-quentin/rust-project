use sea_orm_migration::prelude::*;

use super::organisation::Organisation;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Roles {
    Table,
    Id,
    OrganisationId,
    Name,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::OrganisationId).uuid().not_null())
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Roles::Table, Roles::OrganisationId)
                            .to(Organisation::Table, Organisation::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Roles::Table).to_owned()).await?;

        Ok(())
    }
}
