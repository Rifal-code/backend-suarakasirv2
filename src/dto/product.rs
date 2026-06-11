use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be between 2 and 255 characters"))]
    pub name: String,

    pub price: Decimal,

    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be between 2 and 255 characters"))]
    pub name: Option<String>,

    pub price: Option<Decimal>,

    pub description: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub price: Decimal,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ProductListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}
