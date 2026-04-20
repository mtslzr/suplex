use async_graphql::Object;
use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::entities::promotion::Model;

pub struct PromotionType(pub Model);

#[Object]
impl PromotionType {
    async fn id(&self) -> Uuid {
        self.0.id
    }

    async fn cagematch_id(&self) -> i32 {
        self.0.cagematch_id
    }

    async fn nickname(&self) -> &str {
        &self.0.nickname
    }

    async fn canonical_name(&self) -> Option<&str> {
        self.0.canonical_name.as_deref()
    }

    async fn abbreviation(&self) -> Option<&str> {
        self.0.abbreviation.as_deref()
    }

    async fn country(&self) -> Option<&str> {
        self.0.country.as_deref()
    }

    async fn logo_url(&self) -> Option<&str> {
        self.0.logo_url.as_deref()
    }

    async fn cagematch_url(&self) -> Option<&str> {
        self.0.cagematch_url.as_deref()
    }

    async fn accent_color(&self) -> Option<&str> {
        self.0.accent_color.as_deref()
    }

    async fn enabled(&self) -> bool {
        self.0.enabled
    }

    async fn last_synced_at(&self) -> Option<DateTime<FixedOffset>> {
        self.0.last_synced_at
    }

    async fn created_at(&self) -> DateTime<FixedOffset> {
        self.0.created_at
    }

    async fn updated_at(&self) -> DateTime<FixedOffset> {
        self.0.updated_at
    }
}
