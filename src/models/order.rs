use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub total_amount: Decimal,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub subtotal: Decimal,
}

impl Order {
    pub fn new_id() -> String {
        Uuid::new_v4().to_string()
    }
}

impl OrderItem {
    pub fn new_id() -> String {
        Uuid::new_v4().to_string()
    }
}
