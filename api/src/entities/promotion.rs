use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "promotions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub cagematch_id: i32,
    #[sea_orm(unique)]
    pub nickname: String,
    pub canonical_name: Option<String>,
    pub abbreviation: Option<String>,
    pub country: Option<String>,
    pub logo_url: Option<String>,
    pub cagematch_url: Option<String>,
    pub accent_color: Option<String>,
    pub enabled: bool,
    pub last_synced_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
