use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub price: Decimal,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Product {
    pub fn new_id() -> String {
        Uuid::new_v4().to_string()
    }
}
