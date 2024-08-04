use async_trait::async_trait;
use sea_orm::entity::prelude::*;
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
impl ActiveModelBehavior for ActiveModel {}
