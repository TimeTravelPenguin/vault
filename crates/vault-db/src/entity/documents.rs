use sea_orm::entity::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique)]
    pub path: String,

    pub directory_id: Option<Uuid>,

    pub title: Option<String>,
    pub kind: DocumentKind,
    pub year: Option<i32>,

    pub size_bytes: i64,
    pub modified_at: Option<OffsetDateTime>,
    pub content_hash: Option<String>,

    pub doi: Option<String>,
    pub isbn: Option<String>,
    pub arxiv_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum DocumentKind {
    #[sea_orm(string_value = "textbook")]
    Textbook,

    #[sea_orm(string_value = "paper")]
    Paper,

    #[sea_orm(string_value = "thesis")]
    Thesis,

    #[sea_orm(string_value = "notes")]
    Notes,

    #[sea_orm(string_value = "manual")]
    Manual,

    #[sea_orm(string_value = "other")]
    Other,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::directories::Entity",
        from = "Column::DirectoryId",
        to = "super::directories::Column::Id"
    )]
    Directory,
}

impl Related<super::directories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Directory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
