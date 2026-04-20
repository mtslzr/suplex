use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sync_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub promotion_id: Option<Uuid>,
    pub scope: String,
    pub started_at: DateTimeWithTimeZone,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub status: String,
    pub items_created: i32,
    pub items_updated: i32,
    pub error: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::promotion::Entity",
        from = "Column::PromotionId",
        to = "super::promotion::Column::Id"
    )]
    Promotion,
}

impl Related<super::promotion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Promotion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
