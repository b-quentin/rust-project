use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Collections::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Collections::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Collections::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Fields::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fields::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Fields::CollectionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Fields::ValueType).string().not_null())
                    .col(ColumnDef::new(Fields::ValueName).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-field-collection_id")
                            .from(Fields::Table, Fields::CollectionId)
                            .to(Collections::Table, Collections::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Fields::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Collections::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Collections {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Fields {
    Table,
    Id,
    CollectionId,
    ValueType,
    ValueName,
}


