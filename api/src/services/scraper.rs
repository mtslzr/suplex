use std::sync::Arc;

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::entities::{promotion, sync_log, title};
use crate::external::cagematch::{CagematchClient, parse_promotion_metadata, parse_titles_listing};

#[derive(Debug)]
pub struct ScrapeSummary {
    pub promotion_id: Uuid,
    pub titles_created: i32,
    pub titles_updated: i32,
}

pub async fn scrape_promotion(
    db: &DatabaseConnection,
    cagematch: Arc<CagematchClient>,
    promotion_id: Uuid,
) -> Result<ScrapeSummary, String> {
    let promo = promotion::Entity::find_by_id(promotion_id)
        .one(db)
        .await
        .map_err(|e| format!("db error loading promotion: {e}"))?
        .ok_or_else(|| "promotion not found".to_string())?;

    let log_id = begin_sync_log(db, Some(promotion_id)).await?;

    let result = run_scrape(db, cagematch.as_ref(), promo).await;

    match &result {
        Ok(summary) => {
            finish_sync_log(
                db,
                log_id,
                "success",
                summary.titles_created,
                summary.titles_updated,
                None,
            )
            .await?
        }
        Err(e) => finish_sync_log(db, log_id, "error", 0, 0, Some(e.clone())).await?,
    }

    result
}

async fn run_scrape(
    db: &DatabaseConnection,
    cagematch: &CagematchClient,
    promo: promotion::Model,
) -> Result<ScrapeSummary, String> {
    let now = Utc::now().fixed_offset();

    let promo_html = cagematch.fetch_promotion_page(promo.cagematch_id).await?;
    let meta = parse_promotion_metadata(&promo_html);

    let mut promo_update: promotion::ActiveModel = promo.clone().into();
    if meta.canonical_name.is_some() {
        promo_update.canonical_name = Set(meta.canonical_name);
    }
    if meta.abbreviation.is_some() {
        promo_update.abbreviation = Set(meta.abbreviation);
    }
    if meta.country.is_some() {
        promo_update.country = Set(meta.country);
    }
    if meta.logo_url.is_some() {
        promo_update.logo_url = Set(meta.logo_url);
    }
    promo_update.last_synced_at = Set(Some(now));
    promo_update.updated_at = Set(now);
    let promo = promo_update
        .update(db)
        .await
        .map_err(|e| format!("failed to update promotion: {e}"))?;

    let titles_html = cagematch.fetch_titles_page(promo.cagematch_id).await?;
    let scraped = parse_titles_listing(&titles_html);

    let mut created = 0;
    let mut updated = 0;

    for s in scraped {
        let existing = title::Entity::find()
            .filter(
                Condition::all()
                    .add(title::Column::PromotionId.eq(promo.id))
                    .add(title::Column::CagematchId.eq(s.cagematch_id)),
            )
            .one(db)
            .await
            .map_err(|e| format!("db error loading title: {e}"))?;

        let cagematch_url = Some(cagematch.title_url(s.cagematch_id));

        match existing {
            Some(existing) => {
                let mut model: title::ActiveModel = existing.into();
                model.name = Set(s.name);
                model.is_active = Set(s.is_active);
                model.cagematch_url = Set(cagematch_url);
                model.current_champion_display = Set(s.champion_display);
                model.current_champion_cagematch_id = Set(s.champion_cagematch_id);
                model.current_since_date = Set(s.since_date);
                model.last_synced_at = Set(Some(now));
                model.updated_at = Set(now);
                model
                    .update(db)
                    .await
                    .map_err(|e| format!("failed to update title: {e}"))?;
                updated += 1;
            }
            None => {
                let model = title::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    promotion_id: Set(promo.id),
                    cagematch_id: Set(s.cagematch_id),
                    name: Set(s.name),
                    is_active: Set(s.is_active),
                    cagematch_url: Set(cagematch_url),
                    current_champion_display: Set(s.champion_display),
                    current_champion_cagematch_id: Set(s.champion_cagematch_id),
                    current_since_date: Set(s.since_date),
                    last_synced_at: Set(Some(now)),
                    created_at: Set(now),
                    updated_at: Set(now),
                };
                model
                    .insert(db)
                    .await
                    .map_err(|e| format!("failed to insert title: {e}"))?;
                created += 1;
            }
        }
    }

    Ok(ScrapeSummary {
        promotion_id: promo.id,
        titles_created: created,
        titles_updated: updated,
    })
}

async fn begin_sync_log(
    db: &DatabaseConnection,
    promotion_id: Option<Uuid>,
) -> Result<Uuid, String> {
    let id = Uuid::new_v4();
    let model = sync_log::ActiveModel {
        id: Set(id),
        promotion_id: Set(promotion_id),
        scope: Set("promotion".to_string()),
        started_at: Set(Utc::now().fixed_offset()),
        completed_at: Set(None),
        status: Set("running".to_string()),
        items_created: Set(0),
        items_updated: Set(0),
        error: Set(None),
    };
    model
        .insert(db)
        .await
        .map_err(|e| format!("failed to insert sync_log: {e}"))?;
    Ok(id)
}

async fn finish_sync_log(
    db: &DatabaseConnection,
    id: Uuid,
    status: &str,
    created: i32,
    updated: i32,
    error: Option<String>,
) -> Result<(), String> {
    let existing = sync_log::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("db error loading sync_log: {e}"))?
        .ok_or_else(|| "sync_log row not found".to_string())?;
    let mut model: sync_log::ActiveModel = existing.into();
    model.status = Set(status.to_string());
    model.completed_at = Set(Some(Utc::now().fixed_offset()));
    model.items_created = Set(created);
    model.items_updated = Set(updated);
    model.error = Set(error);
    model
        .update(db)
        .await
        .map_err(|e| format!("failed to update sync_log: {e}"))?;
    Ok(())
}
