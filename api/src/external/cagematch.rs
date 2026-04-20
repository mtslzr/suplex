use std::time::{Duration, Instant};

use chrono::NaiveDate;
use scraper::{Html, Selector};
use tokio::sync::Mutex;

const BASE_URL: &str = "https://www.cagematch.net";
const SOFT_404_MARKER: &str = "Error 404 - Page not found";

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
            base_url: BASE_URL.to_string(),
            rate_limit: Duration::from_millis(rate_limit_ms),
            last_request: Mutex::new(None),
        }
    }

    pub fn promotion_url(&self, cagematch_id: i32) -> String {
        format!("{}/?id=8&nr={}", self.base_url, cagematch_id)
    }

    pub fn titles_url(&self, cagematch_id: i32) -> String {
        format!("{}/?id=8&nr={}&page=9", self.base_url, cagematch_id)
    }

    pub fn title_url(&self, title_cagematch_id: i32) -> String {
        format!("{}/?id=5&nr={}", self.base_url, title_cagematch_id)
    }

    /// Lightweight validation: confirm the promotion page returns HTTP 200
    /// and isn't cagematch's soft 404 (served as HTTP 200 with an error body).
    /// Used during Settings → Add Promotion.
    pub async fn validate_promotion(&self, cagematch_id: i32) -> Result<(), String> {
        self.fetch(&self.promotion_url(cagematch_id))
            .await
            .map(|_| ())
    }

    /// Fetch the main promotion page HTML.
    pub async fn fetch_promotion_page(&self, cagematch_id: i32) -> Result<String, String> {
        self.fetch(&self.promotion_url(cagematch_id)).await
    }

    /// Fetch the titles listing page HTML.
    pub async fn fetch_titles_page(&self, cagematch_id: i32) -> Result<String, String> {
        self.fetch(&self.titles_url(cagematch_id)).await
    }

    async fn fetch(&self, url: &str) -> Result<String, String> {
        self.throttle().await;
        let res = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| format!("failed to reach cagematch.net: {e}"))?;
        if !res.status().is_success() {
            return Err(format!(
                "cagematch.net returned HTTP {} for {}",
                res.status().as_u16(),
                url
            ));
        }
        let body = res
            .text()
            .await
            .map_err(|e| format!("failed to read cagematch.net response: {e}"))?;
        if body.contains(SOFT_404_MARKER) {
            return Err(format!("cagematch.net returned a soft 404 for {url}"));
        }
        Ok(body)
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

#[derive(Debug, Clone, Default)]
pub struct PromotionMetadata {
    pub canonical_name: Option<String>,
    pub abbreviation: Option<String>,
    pub country: Option<String>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ScrapedTitle {
    pub cagematch_id: i32,
    pub name: String,
    pub is_active: bool,
    pub champion_display: Option<String>,
    pub champion_cagematch_id: Option<i32>,
    pub since_date: Option<NaiveDate>,
}

/// Parse the main promotion page for name, abbreviation, country, and logo.
pub fn parse_promotion_metadata(html: &str) -> PromotionMetadata {
    let doc = Html::parse_document(html);

    let (name_from_title, abbrev_from_title) = extract_name_and_abbrev_from_title(&doc);
    let abbreviation = extract_labeled_value(html, "Current abbreviation").or(abbrev_from_title);
    let country = extract_labeled_value(html, "Location");
    let logo_url = extract_logo_url(&doc);

    PromotionMetadata {
        canonical_name: name_from_title,
        abbreviation,
        country,
        logo_url,
    }
}

/// Parse the titles listing page into one `ScrapedTitle` per title row.
/// Rows missing a title link are skipped; rows missing a champion link are
/// kept with `champion_display = None` (vacant / TBD titles).
pub fn parse_titles_listing(html: &str) -> Vec<ScrapedTitle> {
    let doc = Html::parse_document(html);
    let row_sel = Selector::parse("tr").unwrap();
    let link_sel = Selector::parse("a").unwrap();

    let mut titles = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for row in doc.select(&row_sel) {
        let row_text: String = row.text().collect();
        let mut title_id: Option<i32> = None;
        let mut title_name: Option<String> = None;
        let mut champion_id: Option<i32> = None;
        let mut champion_name: Option<String> = None;

        for link in row.select(&link_sel) {
            let href = link.value().attr("href").unwrap_or("");
            let text: String = link.text().collect::<String>().trim().to_string();
            if text.is_empty() {
                continue;
            }
            if let Some(id) = extract_cagematch_nr(href, 5)
                && title_id.is_none()
            {
                title_id = Some(id);
                title_name = Some(text);
            } else if let Some(id) = extract_cagematch_nr(href, 2)
                && champion_id.is_none()
            {
                champion_id = Some(id);
                champion_name = Some(text);
            }
        }

        let (Some(id), Some(name)) = (title_id, title_name) else {
            continue;
        };
        if !seen.insert(id) {
            continue;
        }

        titles.push(ScrapedTitle {
            cagematch_id: id,
            name,
            is_active: true,
            champion_display: champion_name,
            champion_cagematch_id: champion_id,
            since_date: extract_first_date(&row_text),
        });
    }
    titles
}

fn extract_name_and_abbrev_from_title(doc: &Html) -> (Option<String>, Option<String>) {
    let sel = Selector::parse("title").unwrap();
    let Some(el) = doc.select(&sel).next() else {
        return (None, None);
    };
    let raw: String = el.text().collect();
    let head = raw.split(" « ").next().unwrap_or(&raw).trim().to_string();
    if head.is_empty() {
        return (None, None);
    }

    // "All Elite Wrestling (AEW)" → ("All Elite Wrestling", "AEW")
    if let Some(open) = head.rfind(" (")
        && head.ends_with(')')
    {
        let name = head[..open].trim().to_string();
        let abbrev = head[open + 2..head.len() - 1].trim().to_string();
        if !name.is_empty() && !abbrev.is_empty() {
            return (Some(name), Some(abbrev));
        }
    }
    (Some(head), None)
}

/// Pull "Label: Value" from raw HTML, tolerating arbitrary tags between the
/// label and the value (e.g. `<dt>Label:</dt><dd>Value</dd>` or
/// `<div>Label: <a>Value</a></div>`). Stops the value at the next tag or newline.
fn extract_labeled_value(html: &str, label: &str) -> Option<String> {
    let pattern = format!(
        r"{}:\s*(?:<[^>]*>\s*)*([^<\r\n]+?)\s*(?:<|$)",
        regex::escape(label)
    );
    let re = regex::Regex::new(&pattern).ok()?;
    let caps = re.captures(html)?;
    caps.get(1)
        .map(|m| m.as_str().trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_logo_url(doc: &Html) -> Option<String> {
    let sel = Selector::parse("img").unwrap();
    for img in doc.select(&sel) {
        let Some(src) = img.value().attr("src") else {
            continue;
        };
        if !src.contains("/site/main/img/ligen/") {
            continue;
        }
        let absolute = if src.starts_with("http") {
            src.to_string()
        } else if let Some(stripped) = src.strip_prefix('/') {
            format!("{BASE_URL}/{stripped}")
        } else {
            format!("{BASE_URL}/{src}")
        };
        return Some(absolute);
    }
    None
}

fn extract_cagematch_nr(href: &str, entity_id: i32) -> Option<i32> {
    let prefix = format!("?id={entity_id}&nr=");
    let start = href.find(&prefix)?;
    let rest = &href[start + prefix.len()..];
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse::<i32>().ok()
}

fn extract_first_date(text: &str) -> Option<NaiveDate> {
    let re = regex::Regex::new(r"(\d{2})\.(\d{2})\.(\d{4})").ok()?;
    let caps = re.captures(text)?;
    let day = caps.get(1)?.as_str().parse::<u32>().ok()?;
    let month = caps.get(2)?.as_str().parse::<u32>().ok()?;
    let year = caps.get(3)?.as_str().parse::<i32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_promotion_metadata_extracts_name_and_abbreviation_from_title_tag() {
        let html = r#"
            <html>
                <head><title>All Elite Wrestling (AEW) « Promotions Database « CAGEMATCH - The Internet Wrestling Database</title></head>
                <body>
                    <h1>All Elite Wrestling (AEW)</h1>
                    <dl>
                        <dt>Current abbreviation:</dt><dd>AEW</dd>
                        <dt>Location:</dt><dd>Jacksonville, Florida, USA</dd>
                    </dl>
                    <img src="/site/main/img/ligen/normal/2287__20240306-.gif" alt="AEW">
                </body>
            </html>
        "#;
        let meta = parse_promotion_metadata(html);
        assert_eq!(meta.canonical_name.as_deref(), Some("All Elite Wrestling"));
        assert_eq!(meta.abbreviation.as_deref(), Some("AEW"));
        assert_eq!(meta.country.as_deref(), Some("Jacksonville, Florida, USA"));
        assert_eq!(
            meta.logo_url.as_deref(),
            Some("https://www.cagematch.net/site/main/img/ligen/normal/2287__20240306-.gif")
        );
    }

    #[test]
    fn parse_titles_listing_pairs_title_and_champion_links_per_row() {
        let html = r#"
            <table>
                <tr>
                    <td>1</td>
                    <td><a href="?id=5&nr=4331">AEW World Championship</a></td>
                    <td><a href="?id=2&nr=18849">Darby Allin</a></td>
                    <td>15.04.2026 - today (5 days)</td>
                </tr>
                <tr>
                    <td>2</td>
                    <td><a href="?id=5&nr=6550">AEW Continental Championship</a></td>
                    <td><a href="?id=2&nr=2345">Jon Moxley</a></td>
                    <td>12.03.2026 - today (39 days)</td>
                </tr>
            </table>
        "#;
        let titles = parse_titles_listing(html);
        assert_eq!(titles.len(), 2);
        assert_eq!(titles[0].cagematch_id, 4331);
        assert_eq!(titles[0].name, "AEW World Championship");
        assert_eq!(titles[0].champion_display.as_deref(), Some("Darby Allin"));
        assert_eq!(titles[0].champion_cagematch_id, Some(18849));
        assert_eq!(titles[0].since_date, NaiveDate::from_ymd_opt(2026, 4, 15));
        assert_eq!(titles[1].name, "AEW Continental Championship");
        assert_eq!(titles[1].champion_display.as_deref(), Some("Jon Moxley"));
    }

    #[test]
    fn parse_titles_listing_keeps_vacant_rows() {
        let html = r#"
            <table>
                <tr>
                    <td><a href="?id=5&nr=9999">Vacant Title</a></td>
                    <td>(vacant)</td>
                    <td></td>
                </tr>
            </table>
        "#;
        let titles = parse_titles_listing(html);
        assert_eq!(titles.len(), 1);
        assert_eq!(titles[0].champion_display, None);
        assert_eq!(titles[0].champion_cagematch_id, None);
    }
}
