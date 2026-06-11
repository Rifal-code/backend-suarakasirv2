use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AiChatRequest {
    #[validate(length(min = 1, max = 2000, message = "Message must be between 1 and 2000 characters"))]
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AiChatResponse {
    pub reply: String,
}
