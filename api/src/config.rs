/// Application configuration loaded from environment variables.
pub struct Config {
    pub database_url: String,
    pub api_port: u16,
    pub scraper_user_agent: String,
    pub scraper_rate_limit_ms: u64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            api_port: std::env::var("API_PORT")
                .unwrap_or_else(|_| "8083".to_string())
                .parse()
                .expect("API_PORT must be a valid port number"),
            scraper_user_agent: std::env::var("SCRAPER_USER_AGENT").unwrap_or_else(|_| {
                "suplex/0.1 (+https://github.com/mtslzr/suplex)".to_string()
            }),
            scraper_rate_limit_ms: std::env::var("SCRAPER_RATE_LIMIT_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .expect("SCRAPER_RATE_LIMIT_MS must be a valid number"),
        }
    }
}
