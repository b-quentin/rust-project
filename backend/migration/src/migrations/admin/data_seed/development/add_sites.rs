use sea_orm_migration::prelude::*;
use sea_orm::sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum Site {
    Table,
    Id,
    Name,
    Description,
    Domain,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sites = vec![
            (
                Uuid::parse_str("323e4567-e89b-12d3-a456-426614174001").unwrap(),
                "Site Principal",
                "Site vitrine principal de l'entreprise",
                "www.example.com",
            ),
            (
                Uuid::parse_str("323e4567-e89b-12d3-a456-426614174002").unwrap(),
                "Blog Entreprise",
                "Blog officiel de l'entreprise",
                "blog.example.com",
            ),
        ];

        for (id, name, description, domain) in sites {
            let insert_stmt = Query::insert()
                .into_table(Site::Table)
                .columns([
                    Site::Id,
                    Site::Name,
                    Site::Description,
                    Site::Domain,
                    Site::CreatedAt,
                    Site::UpdatedAt,
                ])
                .values_panic([
                    id.into(),
                    name.into(),
                    description.into(),
                    domain.into(),
                    Utc::now().into(),
                    Utc::now().into(),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let site_ids = vec![
            "323e4567-e89b-12d3-a456-426614174001",
            "323e4567-e89b-12d3-a456-426614174002",
        ];

        for uuid in site_ids {
            let delete_stmt = Query::delete()
                .from_table(Site::Table)
                .and_where(Expr::col(Site::Id).eq(Uuid::parse_str(uuid).unwrap()))
                .to_owned();

            manager.exec_stmt(delete_stmt).await?;
        }
        Ok(())
    }
} 