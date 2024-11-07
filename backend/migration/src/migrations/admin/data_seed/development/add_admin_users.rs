use sea_orm_migration::prelude::*;
use sea_orm::sqlx::types::chrono::Utc;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

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
        let admin_users = vec![
            ("223e4567-e89b-12d3-a456-426614174001", "admin1", "AdminFirst1", "AdminLast1", "admin1@example.com", "password123"),
            ("223e4567-e89b-12d3-a456-426614174002", "admin2", "AdminFirst2", "AdminLast2", "admin2@example.com", "password456"),
        ];

        for (uuid, username, first_name, last_name, email, password) in admin_users {
            let hashed_password = hash(password.as_bytes(), DEFAULT_COST)
                .map_err(|e| DbErr::Custom(format!("Failed to hash password: {}", e)))?;

            let insert_stmt = Query::insert()
                .into_table(AdminUsers::Table)
                .columns([
                    AdminUsers::Id,
                    AdminUsers::Username,
                    AdminUsers::FirstName,
                    AdminUsers::LastName,
                    AdminUsers::Email,
                    AdminUsers::Password,
                    AdminUsers::CreatedAt,
                    AdminUsers::UpdatedAt,
                ])
                .values_panic([
                    Uuid::parse_str(uuid).unwrap().into(),
                    username.into(),
                    first_name.into(),
                    last_name.into(),
                    email.into(),
                    hashed_password.into(),
                    Utc::now().into(),
                    Utc::now().into(),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let admin_user_ids = vec![
            "223e4567-e89b-12d3-a456-426614174001",
            "223e4567-e89b-12d3-a456-426614174002",
        ];

        for uuid in admin_user_ids {
            let delete_stmt = Query::delete()
                .from_table(AdminUsers::Table)
                .and_where(Expr::col(AdminUsers::Id).eq(Uuid::parse_str(uuid).unwrap()))
                .to_owned();

            manager.exec_stmt(delete_stmt).await?;
        }

        Ok(())
    }
}

