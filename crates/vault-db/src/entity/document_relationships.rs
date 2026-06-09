use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "document_relationships")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub from_document_id: Uuid,

    #[sea_orm(primary_key, auto_increment = false)]
    pub to_document_id: Uuid,

    #[sea_orm(primary_key, auto_increment = false)]
    pub kind: RelationshipKind,

    pub note: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum RelationshipKind {
    #[sea_orm(string_value = "cites")]
    Cites,

    #[sea_orm(string_value = "supplements")]
    Supplements,

    #[sea_orm(string_value = "solution_manual_for")]
    SolutionManualFor,

    #[sea_orm(string_value = "newer_edition_of")]
    NewerEditionOf,

    #[sea_orm(string_value = "duplicate_of")]
    DuplicateOf,

    #[sea_orm(string_value = "related_to")]
    RelatedTo,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
