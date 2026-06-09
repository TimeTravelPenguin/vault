use sea_orm::entity::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm::model]
#[sea_orm(table_name = "directories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique)]
    pub path: String,

    pub display_name: Option<String>,
    pub recursive: bool,
    pub enabled: bool,

    pub last_scanned_at: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::documents::Entity")]
    Documents,
}

impl Related<super::documents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Documents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
