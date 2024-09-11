use sea_orm_migration::prelude::*;
use sea_orm::sea_query::{Query, Expr};
use chrono::Utc;
use uuid::Uuid;
use std::env;

#[derive(DeriveMigrationName)]
pub struct SeedUsers;

#[async_trait::async_trait]
impl MigrationTrait for SeedUsers {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if env::var("APP_ENV").unwrap_or_default() == "development" {
            let now = Utc::now().naive_utc();
            let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string(); // Convert NaiveDateTime to SQL-compatible string

            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(Users::Table)
                        .columns([
                            Users::Id,
                            Users::Username,
                            Users::Email,
                            Users::PasswordHash,
                            Users::CreatedAt,
                            Users::UpdatedAt,
                            Users::IsActive,
                        ])
                        .values_panic([
                            Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap().to_string().into(),
                            "admin".into(),
                            "admin@example.com".into(),
                            "hashed_password1".into(),
                            now_str.clone().into(),
                            now_str.clone().into(),
                            true.into(),
                        ])
                        .values_panic([
                            Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap().to_string().into(),
                            "user1".into(),
                            "user1@example.com".into(),
                            "hashed_password2".into(),
                            now_str.clone().into(),
                            now_str.clone().into(),
                            true.into(),
                        ])
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

   async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if env::var("APP_ENV").unwrap_or_default() == "development" {
            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(Users::Table)
                        .and_where(
                            Expr::col(Users::Id)
                                .eq("11111111-1111-1111-1111-111111111111")
                        )
                        .and_where(
                            Expr::col(Users::Id)
                                .eq("22222222-2222-2222-2222-222222222222")
                        )
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
    IsActive,
    LastLogin,
}
