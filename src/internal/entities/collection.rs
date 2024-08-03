use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ActiveValue, DbBackend, Statement};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "collections")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::field::Entity")]
    Field,
}

impl Related<super::field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Field.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if let ActiveValue::Set(ref current_name) = self.name {
            let sanitized_name = current_name.replace("'", "''");

            // Check if the table exists
            let check_table_stmt = format!(
                "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = '{}')",
                sanitized_name
            );
            let result = db.query_one(Statement::from_string(DbBackend::Postgres, check_table_stmt)).await?;

            if let Some(row) = result {
                let exists: bool = row.try_get("", "exists")?;
                if exists {
                    // Save the current name in old_name if the table exists
                    self.old_name = ActiveValue::Set(Some(sanitized_name.clone()));
                } else {
                    // Table does not exist, create it
                    let create_table_stmt = format!(
                        "CREATE TABLE {} (id SERIAL PRIMARY KEY, name VARCHAR(255), other_field VARCHAR(255))",
                        sanitized_name
                    );
                    db.execute(Statement::from_string(DbBackend::Postgres, create_table_stmt)).await?;
                }
            }

            // Rename the table if old_name is set and different from current name
            if let ActiveValue::Set(Some(ref old_name)) = self.old_name {
                if old_name != &sanitized_name {
                    let rename_table_stmt = format!(
                        "ALTER TABLE {} RENAME TO {}",
                        old_name, sanitized_name
                    );
                    db.execute(Statement::from_string(DbBackend::Postgres, rename_table_stmt)).await?;
                }
            }
        }

        Ok(self)
    }
}
