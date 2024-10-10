use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminUsersAdminRoles {
    Table,
    AdminUserId,
    RoleAdminId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user_role_assignments = vec![
            ("223e4567-e89b-12d3-a456-426614174001", "123e4567-e89b-12d3-a456-426614174000"),
            ("223e4567-e89b-12d3-a456-426614174002", "123e4567-e89b-12d3-a456-426614174001"),
        ];

        for (admin_user_id, role_admin_id) in user_role_assignments {
            let insert_stmt = Query::insert()
                .into_table(AdminUsersAdminRoles::Table)
                .columns([
                    AdminUsersAdminRoles::AdminUserId,
                    AdminUsersAdminRoles::RoleAdminId,
                ])
                .values_panic([
                    Uuid::parse_str(admin_user_id).unwrap().into(),
                    Uuid::parse_str(role_admin_id).unwrap().into(),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user_group_assignments = vec![
            ("223e4567-e89b-12d3-a456-426614174001", "123e4567-e89b-12d3-a456-426614174000"),
            ("223e4567-e89b-12d3-a456-426614174002", "123e4567-e89b-12d3-a456-426614174001"),
        ];

        for (admin_user_id, role_admin_id) in user_group_assignments {
            let delete_stmt = Query::delete()
                .from_table(AdminUsersAdminRoles::Table)
                .and_where(
                    Expr::col(AdminUsersAdminRoles::AdminUserId)
                        .eq(Uuid::parse_str(admin_user_id).unwrap())
                )
                .and_where(
                    Expr::col(AdminUsersAdminRoles::RoleAdminId)
                        .eq(Uuid::parse_str(role_admin_id).unwrap())
                )
                .to_owned();

            manager.exec_stmt(delete_stmt).await?;
        }

        Ok(())
    }
}

