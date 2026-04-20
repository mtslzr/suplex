use async_graphql::Object;
use chrono::{DateTime, FixedOffset, NaiveDate};
use uuid::Uuid;

use crate::entities::title::Model;

pub struct TitleType(pub Model);

#[Object]
impl TitleType {
    async fn id(&self) -> Uuid {
        self.0.id
    }

    async fn promotion_id(&self) -> Uuid {
        self.0.promotion_id
    }

    async fn cagematch_id(&self) -> i32 {
        self.0.cagematch_id
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn is_active(&self) -> bool {
        self.0.is_active
    }

    async fn cagematch_url(&self) -> Option<&str> {
        self.0.cagematch_url.as_deref()
    }

    async fn current_champion_display(&self) -> Option<&str> {
        self.0.current_champion_display.as_deref()
    }

    async fn current_champion_cagematch_id(&self) -> Option<i32> {
        self.0.current_champion_cagematch_id
    }

    async fn current_since_date(&self) -> Option<NaiveDate> {
        self.0.current_since_date
    }

    async fn last_synced_at(&self) -> Option<DateTime<FixedOffset>> {
        self.0.last_synced_at
    }
}
