use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::entities::{promotion, title};

use super::types::promotion::PromotionType;
use super::types::title::TitleType;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Health check
    async fn health(&self) -> &str {
        "ok"
    }

    /// All tracked promotions, ordered by nickname.
    async fn promotions(&self, ctx: &Context<'_>) -> Result<Vec<PromotionType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let rows = promotion::Entity::find()
            .order_by_asc(promotion::Column::Nickname)
            .all(db)
            .await?;
        Ok(rows.into_iter().map(PromotionType).collect())
    }

    /// Single promotion by id.
    async fn promotion(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<PromotionType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let row = promotion::Entity::find_by_id(id).one(db).await?;
        Ok(row.map(PromotionType))
    }

    /// Titles across all promotions, optionally filtered. Results are ordered
    /// by promotion (via id) then by title name for stable display.
    async fn titles(
        &self,
        ctx: &Context<'_>,
        promotion_id: Option<Uuid>,
        active_only: Option<bool>,
    ) -> Result<Vec<TitleType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut condition = Condition::all();
        if let Some(pid) = promotion_id {
            condition = condition.add(title::Column::PromotionId.eq(pid));
        }
        if active_only.unwrap_or(false) {
            condition = condition.add(title::Column::IsActive.eq(true));
        }
        let rows = title::Entity::find()
            .filter(condition)
            .order_by_asc(title::Column::PromotionId)
            .order_by_asc(title::Column::Name)
            .all(db)
            .await?;
        Ok(rows.into_iter().map(TitleType).collect())
    }
}
