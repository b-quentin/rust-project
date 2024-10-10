use sea_orm_migration::prelude::*;
use super::users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Address {
    Table,
    Id,
    UserId,
    Street,
    City,
    PostalCode,
    Country,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Address::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Address::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Address::UserId).uuid().not_null())
                    .col(ColumnDef::new(Address::Street).string().not_null())
                    .col(ColumnDef::new(Address::City).string().not_null())
                    .col(ColumnDef::new(Address::PostalCode).string().not_null())
                    .col(ColumnDef::new(Address::Country).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Address::Table, Address::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Address::Table).to_owned()).await?;

        Ok(())
    }
}
