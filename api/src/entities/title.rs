use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "titles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub cagematch_id: i32,
    pub name: String,
    pub is_active: bool,
    pub cagematch_url: Option<String>,
    pub current_champion_display: Option<String>,
    pub current_champion_cagematch_id: Option<i32>,
    pub current_since_date: Option<Date>,
    pub last_synced_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::promotion::Entity",
        from = "Column::PromotionId",
        to = "super::promotion::Column::Id",
        on_delete = "Cascade"
    )]
    Promotion,
}

impl Related<super::promotion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Promotion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
