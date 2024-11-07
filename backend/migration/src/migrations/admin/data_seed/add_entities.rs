use sea_orm_migration::prelude::*;
use uuid::Uuid;
use sea_orm::sqlx::types::chrono::Utc;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminEntities {
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
        // Insertion d'exemples d'entités avec UUIDs
        let entities = vec![
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174110").unwrap(), "/admin/dashboard", "Represents the Admin space."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174111").unwrap(), "/admin/dashboard/users", "Represents the Users page."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174112").unwrap(), "Ressource::User", "Represents the User resource."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174113").unwrap(), "Ressource::Invoice", "Represents the Invoice resource."),
        ];

        // Insérer les entités avec les nouveaux UUIDs
        for (id, name, description) in entities {
            let insert_stmt = Query::insert()
                .into_table(AdminEntities::Table)
                .columns([
                    AdminEntities::Id,
                    AdminEntities::Name,
                    AdminEntities::Description,
                    AdminEntities::CreatedAt,
                    AdminEntities::UpdatedAt,
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
        let entity_names = vec![
            "/admin/dashboard",
            "/admin/dashboard/users",
            "Ressource::User",
            "Ressource::Invoice",
        ];

        // Suppression des entités insérées
        for name in entity_names {
            let delete_stmt = Query::delete()
                .from_table(AdminEntities::Table)
                .and_where(Expr::col(AdminEntities::Name).eq(name))
                .to_owned();

            manager.exec_stmt(delete_stmt).await?;
        }

        Ok(())
    }
}