/// Application configuration loaded from environment variables.
pub struct Config {
    pub database_url: String,
    pub api_port: u16,
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
        }
    }
}
