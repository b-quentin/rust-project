use sea_orm_migration::prelude::*;
use sea_orm::sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminActions {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // UUIDs personnalisés avec un nouveau format cohérent
        let permissions = vec![
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174100").unwrap(), "can_create", "Allows the user to create a new resource."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174101").unwrap(), "can_read", "Allows the user to read or view resources."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174102").unwrap(), "can_update", "Allows the user to update or modify an existing resource."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174103").unwrap(), "can_delete", "Allows the user to delete an existing resource."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174104").unwrap(), "can_list", "Allows the user to list all resources without viewing their details."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174105").unwrap(), "can_upload", "Allows the user to upload files or resources."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174106").unwrap(), "can_download", "Allows the user to download files or resources."),
        ];

        // Insérer les permissions avec les nouveaux UUIDs
        for (id, name, description) in permissions {
            let insert_stmt = Query::insert()
                .into_table(AdminActions::Table)
                .columns([
                    AdminActions::Id,
                    AdminActions::Name,
                    AdminActions::Description,
                    AdminActions::CreatedAt,
                    AdminActions::UpdatedAt,
                ])
                .values_panic([
                    id.into(),
                    name.into(),
                    description.into(),
                    Utc::now().into(),
                    Utc::now().into(),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let permission_names = vec![
            "can_create",
            "can_read",
            "can_update",
            "can_delete",
            "can_list",
            "can_upload",
            "can_download",
        ];

        for name in permission_names {
            let delete_stmt = Query::delete()
                .from_table(AdminActions::Table)
                .and_where(Expr::col(AdminActions::Name).eq(name))
                .to_owned();

            manager.exec_stmt(delete_stmt).await?;
        }
        Ok(())
    }
}

