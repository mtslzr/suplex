use std::sync::Arc;

use async_graphql::{Context, InputObject, Object, Result, SimpleObject};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::entities::promotion;
use crate::external::cagematch::CagematchClient;
use crate::services::scraper;

use super::types::promotion::PromotionType;

#[derive(InputObject)]
pub struct AddPromotionInput {
    pub nickname: String,
    pub cagematch_id: i32,
    pub accent_color: Option<String>,
}

#[derive(InputObject)]
pub struct UpdatePromotionInput {
    pub nickname: Option<String>,
    pub accent_color: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(SimpleObject)]
pub struct ScrapeResult {
    pub promotion_id: Uuid,
    pub titles_created: i32,
    pub titles_updated: i32,
    pub error: Option<String>,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Add a promotion by cagematch ID. Validates that the ID resolves on
    /// cagematch.net; full metadata is populated by the scraper.
    async fn add_promotion(
        &self,
        ctx: &Context<'_>,
        input: AddPromotionInput,
    ) -> Result<PromotionType> {
        let db = ctx.data::<DatabaseConnection>()?;
        let cagematch = ctx.data::<Arc<CagematchClient>>()?;

        cagematch
            .validate_promotion(input.cagematch_id)
            .await
            .map_err(async_graphql::Error::new)?;

        let now = Utc::now().fixed_offset();

        let model = promotion::ActiveModel {
            id: Set(Uuid::new_v4()),
            cagematch_id: Set(input.cagematch_id),
            nickname: Set(input.nickname),
            canonical_name: Set(None),
            abbreviation: Set(None),
            country: Set(None),
            logo_url: Set(None),
            cagematch_url: Set(Some(cagematch.promotion_url(input.cagematch_id))),
            accent_color: Set(input.accent_color),
            enabled: Set(true),
            last_synced_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let result = model.insert(db).await?;
        Ok(PromotionType(result))
    }

    /// Update a promotion's nickname, accent color, or enabled flag.
    async fn update_promotion(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdatePromotionInput,
    ) -> Result<PromotionType> {
        let db = ctx.data::<DatabaseConnection>()?;

        let existing = promotion::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Promotion not found"))?;

        let mut model: promotion::ActiveModel = existing.into();
        model.updated_at = Set(Utc::now().fixed_offset());

        if let Some(nickname) = input.nickname {
            model.nickname = Set(nickname);
        }
        if let Some(accent) = input.accent_color {
            model.accent_color = Set(Some(accent));
        }
        if let Some(enabled) = input.enabled {
            model.enabled = Set(enabled);
        }

        let result = model.update(db).await?;
        Ok(PromotionType(result))
    }

    /// Remove a promotion and cascading rows.
    async fn remove_promotion(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let result = promotion::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    /// Scrape a single promotion: refresh metadata and upsert its titles +
    /// current champions from cagematch.net.
    async fn scrape_promotion(&self, ctx: &Context<'_>, id: Uuid) -> Result<ScrapeResult> {
        let db = ctx.data::<DatabaseConnection>()?;
        let cagematch = ctx.data::<Arc<CagematchClient>>()?;

        match scraper::scrape_promotion(db, cagematch.clone(), id).await {
            Ok(summary) => Ok(ScrapeResult {
                promotion_id: summary.promotion_id,
                titles_created: summary.titles_created,
                titles_updated: summary.titles_updated,
                error: None,
            }),
            Err(e) => Ok(ScrapeResult {
                promotion_id: id,
                titles_created: 0,
                titles_updated: 0,
                error: Some(e),
            }),
        }
    }

    /// Scrape every enabled promotion sequentially. Rate limiter serializes
    /// outbound requests; failures on one promotion do not abort the rest.
    async fn scrape_all_promotions(&self, ctx: &Context<'_>) -> Result<Vec<ScrapeResult>> {
        use sea_orm::{ColumnTrait, QueryFilter};

        let db = ctx.data::<DatabaseConnection>()?;
        let cagematch = ctx.data::<Arc<CagematchClient>>()?;

        let enabled = promotion::Entity::find()
            .filter(promotion::Column::Enabled.eq(true))
            .all(db)
            .await?;

        let mut out = Vec::with_capacity(enabled.len());
        for p in enabled {
            match scraper::scrape_promotion(db, cagematch.clone(), p.id).await {
                Ok(summary) => out.push(ScrapeResult {
                    promotion_id: summary.promotion_id,
                    titles_created: summary.titles_created,
                    titles_updated: summary.titles_updated,
                    error: None,
                }),
                Err(e) => out.push(ScrapeResult {
                    promotion_id: p.id,
                    titles_created: 0,
                    titles_updated: 0,
                    error: Some(e),
                }),
            }
        }
        Ok(out)
    }
}
