pub mod ai;
pub mod auth;
pub mod dashboard;
pub mod feedback;
pub mod order;
pub mod product;

pub use order::OrderItemResponse;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn success(
        message: impl Into<String>,
        data: Vec<T>,
        total: i64,
        page: u32,
        limit: u32,
    ) -> Self {
        Self {
            success: true,
            message: message.into(),
            data,
            total,
            page,
            limit,
        }
    }
}
