use async_graphql::{Context, Object, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use uuid::Uuid;

use crate::entities::promotion;

use super::types::promotion::PromotionType;

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
}
