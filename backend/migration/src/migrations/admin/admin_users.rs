use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminUsers {
    Table,
    Id,
    Username,
    FirstName,
    LastName,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUsers::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminUsers::Username).string().not_null())
                    .col(ColumnDef::new(AdminUsers::FirstName).string().not_null())
                    .col(ColumnDef::new(AdminUsers::LastName).string().not_null())
                    .col(ColumnDef::new(AdminUsers::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(AdminUsers::Password).string().not_null())
                    .col(ColumnDef::new(AdminUsers::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AdminUsers::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AdminUsers::Table).to_owned()).await?;
        Ok(())
    }
}
