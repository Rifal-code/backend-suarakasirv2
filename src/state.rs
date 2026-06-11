use sqlx::MySqlPool;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub config: Config,
}

impl AppState {
    pub fn new(db: MySqlPool, config: Config) -> Self {
        Self { db, config }
    }
}
