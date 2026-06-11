use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub ai_api_key: String,
    pub ai_api_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        Ok(Self {
            host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("APP_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse::<u16>()
                .map_err(|_| "Invalid APP_PORT value")?,
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set")?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET must be set")?,
            ai_api_key: env::var("AI_API_KEY").unwrap_or_default(),
            ai_api_url: env::var("AI_API_URL").unwrap_or_default(),
        })
    }
}
