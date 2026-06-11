use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct OrderItemRequest {
    pub product_id: String,

    #[validate(range(min = 1, message = "Quantity must be greater than 0"))]
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(length(min = 1, message = "Order must have at least one item"))]
    pub items: Vec<OrderItemRequest>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderRequest {
    #[validate(length(min = 1, message = "Order must have at least one item"))]
    pub items: Vec<OrderItemRequest>,

    pub status: Option<i8>,
}

#[derive(Debug, Serialize, Clone)]
pub struct OrderItemResponse {
    pub id: String,
    pub product_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub subtotal: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct OrderResponse {
    pub id: String,
    pub total_amount: Decimal,
    pub status: i8,
    pub items: Vec<OrderItemResponse>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct OrderListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<i8>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}
