use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminUsersPermissionsEntities {
    Table,
    UserId,
    PermissionId,
    EntityId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user_id = Uuid::parse_str("223e4567-e89b-12d3-a456-426614174002").unwrap();
        let permission_ids = vec![
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174100").unwrap(), // can_create
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174101").unwrap(), // can_read
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174102").unwrap(), // can_update
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174103").unwrap(), // can_delete
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174104").unwrap(), // can_list
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174105").unwrap(), // can_upload
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174106").unwrap(), // can_download
        ];
        let entity_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174110").unwrap();

        for permission_id in permission_ids {
            let insert_stmt = Query::insert()
                .into_table(AdminUsersPermissionsEntities::Table)
                .columns([
                    AdminUsersPermissionsEntities::UserId,
                    AdminUsersPermissionsEntities::PermissionId,
                    AdminUsersPermissionsEntities::EntityId,
                ])
                .values_panic([
                    user_id.into(),
                    permission_id.into(),
                    entity_id.into(),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let role_id = Uuid::parse_str("223e4567-e89b-12d3-a456-426614174002").unwrap();
        let permission_ids = vec![
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174100").unwrap(), // can_create
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174101").unwrap(), // can_read
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174102").unwrap(), // can_update
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174103").unwrap(), // can_delete
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174104").unwrap(), // can_list
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174105").unwrap(), // can_upload
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174106").unwrap(), // can_download
        ];
        let entity_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174110").unwrap();

        for permission_id in permission_ids {
            let delete_stmt = Query::delete()
                .from_table(AdminUsersPermissionsEntities::Table)
                .and_where(
                    Expr::col(AdminUsersPermissionsEntities::UserId)
                        .eq(role_id)
                )
                .and_where(
                    Expr::col(AdminUsersPermissionsEntities::PermissionId)
                        .eq(permission_id)
                )
                .and_where(
                    Expr::col(AdminUsersPermissionsEntities::EntityId)
                        .eq(entity_id)
                )
                .to_owned();

            manager.exec_stmt(delete_stmt).await?;
        }
        Ok(())
    }
}

