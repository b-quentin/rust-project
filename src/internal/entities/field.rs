use sea_orm::entity::prelude::*;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fields")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub value_type: String,
    pub value_name: String,
    pub collection_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::collection::Entity"
        from = "Column::CollectionId",
        to = "super::collection::Column::Id"
    )]
    Collection,
}

impl Related<super::collection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collection.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::collection::Relation::Field.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
