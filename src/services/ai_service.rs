use reqwest::Client;
use serde_json::json;

use crate::{
    dto::ai::{AiChatRequest, AiChatResponse},
    errors::AppError,
};

pub struct AiService {
    client: Client,
    api_key: String,
    api_url: String,
}

impl AiService {
    pub fn new(api_key: String, api_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_url,
        }
    }

    pub async fn chat(&self, req: AiChatRequest) -> Result<AiChatResponse, AppError> {
        if self.api_key.is_empty() || self.api_url.is_empty() {
            return Err(AppError::InternalServerError(
                "AI service is not configured".to_string(),
            ));
        }

        let payload = json!({
            "contents": [
                {
                    "parts": [
                        { "text": req.message }
                    ]
                }
            ]
        });

        let url = format!("{}?key={}", self.api_url, self.api_key);

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("AI request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::InternalServerError(format!(
                "AI service returned error: {}",
                response.status()
            )));
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse AI response: {}", e)))?;

        let reply = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("No response from AI")
            .to_string();

        Ok(AiChatResponse { reply })
    }
}
