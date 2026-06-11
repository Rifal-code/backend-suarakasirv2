use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFeedbackRequest {
    #[validate(length(min = 3, max = 1000, message = "Message must be between 3 and 1000 characters"))]
    pub message: String,

    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateFeedbackRequest {
    #[validate(length(min = 3, max = 1000, message = "Message must be between 3 and 1000 characters"))]
    pub message: Option<String>,

    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FeedbackResponse {
    pub id: String,
    pub user_name: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct FeedbackListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}
