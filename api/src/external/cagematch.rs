use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Minimal, rate-limited HTTP client for cagematch.net.
///
/// v0.2 only uses this to validate that a promotion ID resolves to a 200 page.
/// Full HTML parsing lives in v0.3 alongside the scraper.
pub struct CagematchClient {
    http: reqwest::Client,
    base_url: String,
    rate_limit: Duration,
    last_request: Mutex<Option<Instant>>,
}

impl CagematchClient {
    pub fn new(user_agent: &str, rate_limit_ms: u64) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build cagematch HTTP client");
        Self {
            http,
            base_url: "https://www.cagematch.net".to_string(),
            rate_limit: Duration::from_millis(rate_limit_ms),
            last_request: Mutex::new(None),
        }
    }

    /// Canonical public URL for a promotion record.
    pub fn promotion_url(&self, cagematch_id: i32) -> String {
        format!("{}/?id=8&nr={}", self.base_url, cagematch_id)
    }

    /// Lightweight validation: confirm the promotion page returns HTTP 200
    /// and isn't cagematch's soft 404 (served as HTTP 200 with an error body).
    /// Whether the ID maps to an actual promotion (vs some other entity) is
    /// checked during the v0.3 full scrape.
    pub async fn validate_promotion(&self, cagematch_id: i32) -> Result<(), String> {
        self.throttle().await;
        let url = self.promotion_url(cagematch_id);
        let res = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("failed to reach cagematch.net: {e}"))?;
        if !res.status().is_success() {
            return Err(format!(
                "cagematch.net returned HTTP {} for id={cagematch_id}",
                res.status().as_u16()
            ));
        }
        let body = res
            .text()
            .await
            .map_err(|e| format!("failed to read cagematch.net response: {e}"))?;
        if body.contains("Error 404 - Page not found") {
            return Err(format!(
                "no promotion found on cagematch.net for id={cagematch_id}"
            ));
        }
        Ok(())
    }

    /// Block until the configured rate-limit window has elapsed since the
    /// previous request. Serializes outbound traffic to one request per window.
    async fn throttle(&self) {
        let mut last = self.last_request.lock().await;
        if let Some(t) = *last {
            let elapsed = t.elapsed();
            if elapsed < self.rate_limit {
                tokio::time::sleep(self.rate_limit - elapsed).await;
            }
        }
        *last = Some(Instant::now());
    }
}
