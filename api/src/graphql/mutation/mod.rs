use async_graphql::{Context, InputObject, Object, Result};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::entities::promotion;

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

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Add a promotion by cagematch ID. Canonical name, country, logo are
    /// filled in on the next scrape.
    async fn add_promotion(
        &self,
        ctx: &Context<'_>,
        input: AddPromotionInput,
    ) -> Result<PromotionType> {
        let db = ctx.data::<DatabaseConnection>()?;
        let now = Utc::now().fixed_offset();

        let model = promotion::ActiveModel {
            id: Set(Uuid::new_v4()),
            cagematch_id: Set(input.cagematch_id),
            nickname: Set(input.nickname),
            canonical_name: Set(None),
            abbreviation: Set(None),
            country: Set(None),
            logo_url: Set(None),
            cagematch_url: Set(Some(format!(
                "https://www.cagematch.net/?id=8&nr={}",
                input.cagematch_id
            ))),
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

    /// Remove a promotion and any linked sync logs.
    async fn remove_promotion(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let result = promotion::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }
}
