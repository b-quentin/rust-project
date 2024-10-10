use sea_orm_migration::prelude::*;
use sea_orm::sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum Users {
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
        let users = vec![
            ("123e4567-e89b-12d3-a456-426614174001", "user1", "First1", "Last1", "user1@example.com", "hashedpassword1"),
            ("123e4567-e89b-12d3-a456-426614174002", "user2", "First2", "Last2", "user2@example.com", "hashedpassword2"),
            ("123e4567-e89b-12d3-a456-426614174003", "user3", "First3", "Last3", "user3@example.com", "hashedpassword3"),
            ("123e4567-e89b-12d3-a456-426614174004", "user4", "First4", "Last4", "user4@example.com", "hashedpassword4"),
            ("123e4567-e89b-12d3-a456-426614174005", "user5", "First5", "Last5", "user5@example.com", "hashedpassword5"),
            ("123e4567-e89b-12d3-a456-426614174006", "user6", "First6", "Last6", "user6@example.com", "hashedpassword6"),
            ("123e4567-e89b-12d3-a456-426614174007", "user7", "First7", "Last7", "user7@example.com", "hashedpassword7"),
            ("123e4567-e89b-12d3-a456-426614174008", "user8", "First8", "Last8", "user8@example.com", "hashedpassword8"),
            ("123e4567-e89b-12d3-a456-426614174009", "user9", "First9", "Last9", "user9@example.com", "hashedpassword9"),
        ];

        for (uuid, username, first_name, last_name, email, password) in users {
            let insert_stmt = Query::insert()
                .into_table(Users::Table)
                .columns([
                    Users::Id,
                    Users::Username,
                    Users::FirstName,
                    Users::LastName,
                    Users::Email,
                    Users::Password,
                    Users::CreatedAt,
                    Users::UpdatedAt,
                ])
                .values_panic([
                    Uuid::parse_str(uuid).unwrap().into(),
                    username.into(),
                    first_name.into(),
                    last_name.into(),
                    email.into(),
                    password.into(),
                    Utc::now().into(),
                    Utc::now().into(),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user_ids = vec![
            "123e4567-e89b-12d3-a456-426614174001",
            "123e4567-e89b-12d3-a456-426614174002",
            "123e4567-e89b-12d3-a456-426614174003",
            "123e4567-e89b-12d3-a456-426614174004",
            "123e4567-e89b-12d3-a456-426614174005",
            "123e4567-e89b-12d3-a456-426614174006",
            "123e4567-e89b-12d3-a456-426614174007",
            "123e4567-e89b-12d3-a456-426614174008",
            "123e4567-e89b-12d3-a456-426614174009",
        ];

        for uuid in user_ids {
            let delete_stmt = Query::delete()
                .from_table(Users::Table)
                .and_where(Expr::col(Users::Id).eq(Uuid::parse_str(uuid).unwrap()))
                .to_owned();

            manager.exec_stmt(delete_stmt).await?;
        }

        Ok(())
    }
}

