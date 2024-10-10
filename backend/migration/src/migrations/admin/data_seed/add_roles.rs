use sea_orm::sqlx::types::chrono::Utc;
use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminRoles {
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
        let roles = vec![
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(), "Admins", "Users with full access to the platform, able to manage all functionalities."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174001").unwrap(), "Product Managers", "Responsible for managing the product catalog (add, remove, update products)."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174002").unwrap(), "Order Processing", "Users who handle customer orders and order-related tasks."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174003").unwrap(), "Customer Support", "Handles customer relations, including support and return management."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174004").unwrap(), "Marketing", "Responsible for advertising campaigns, promotions, and digital marketing efforts."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174005").unwrap(), "Inventory Managers", "Manages stock levels, inventory control, and reordering processes."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174006").unwrap(), "Sales", "Responsible for sales, customer relationships (B2B), and special offers."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174007").unwrap(), "Finance", "Handles payments, invoicing, and financial reporting."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174008").unwrap(), "Logistics", "Responsible for managing shipments, deliveries, and overall logistics."),
            (Uuid::parse_str("123e4567-e89b-12d3-a456-426614174009").unwrap(), "Developers", "Technical team maintaining and improving the e-commerce platform."),
        ];

        for (id, name, description) in roles {
            let insert_stmt = Query::insert()
                .into_table(AdminRoles::Table)
                .columns([AdminRoles::Id, AdminRoles::Name, AdminRoles::Description, AdminRoles::CreatedAt, AdminRoles::UpdatedAt])
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
        let group_ids = vec![
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174001").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174002").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174003").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174004").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174005").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174006").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174007").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174008").unwrap(),
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174009").unwrap(),
        ];

        for id in group_ids {
            let delete_stmt = Query::delete()
                .from_table(AdminRoles::Table)
                .and_where(Expr::col(AdminRoles::Id).eq(id))
                .to_owned();

            manager.exec_stmt(delete_stmt).await?;
        }
        Ok(())
    }
}

