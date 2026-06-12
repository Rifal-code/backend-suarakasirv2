use reqwest::Client;
use serde_json::json;
use strsim::jaro_winkler;

use crate::{
    dto::ai::{
        AiChatRequest, AiChatResponse, MatchedOrderItem, ParseOrderRequest, ParseOrderResponse,
    },
    errors::AppError,
    models::Product,
    repositories::ProductRepository,
};

/// Minimum confidence score to be considered a match without needing user confirmation.
const CONFIDENCE_THRESHOLD: f64 = 0.80;

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

    /// Parse raw voice order items, fuzzy-match each to available products for the user.
    /// Products are loaded from DB — Gemini never receives the product list.
    pub async fn parse_order(
        &self,
        req: ParseOrderRequest,
        product_repo: &ProductRepository,
        user_id: &str,
    ) -> Result<ParseOrderResponse, AppError> {
        if req.items.is_empty() {
            return Err(AppError::ValidationError(
                "items tidak boleh kosong".to_string(),
            ));
        }

        // Load all active products once
        let products = product_repo.find_all_active_for_user(user_id).await?;

        if products.is_empty() {
            return Err(AppError::NotFound(
                "Tidak ada produk yang terdaftar untuk akun ini".to_string(),
            ));
        }

        let mut matched_items: Vec<MatchedOrderItem> = Vec::with_capacity(req.items.len());

        for raw in &req.items {
            if raw.q <= 0 {
                return Err(AppError::ValidationError(format!(
                    "Jumlah untuk '{}' harus lebih dari 0",
                    raw.n
                )));
            }

            let (best_product, confidence) = best_match(&raw.n, &products);

            matched_items.push(MatchedOrderItem {
                product_id: best_product.id.clone(),
                name: best_product.name.clone(),
                input_name: raw.n.clone(),
                quantity: raw.q,
                unit_price: best_product.price,
                confidence,
                needs_confirmation: confidence < CONFIDENCE_THRESHOLD,
            });
        }

        Ok(ParseOrderResponse {
            items: matched_items,
        })
    }
}

/// Returns the best-matching product and its confidence score [0.0, 1.0].
fn best_match<'a>(input: &str, products: &'a [Product]) -> (&'a Product, f64) {
    let input_lower = input.to_lowercase();

    let mut best_idx = 0usize;
    let mut best_score = 0.0f64;

    for (i, product) in products.iter().enumerate() {
        let name_lower = product.name.to_lowercase();

        // Primary: Jaro-Winkler similarity
        let jw = jaro_winkler(&input_lower, &name_lower);

        // Bonus: exact substring match gives a boost
        let score = if name_lower.contains(&input_lower) || input_lower.contains(&name_lower) {
            (jw + 1.0) / 2.0 // average with 1.0 for substring bonus
        } else {
            jw
        };

        if score > best_score {
            best_score = score;
            best_idx = i;
        }
    }

    // Round to 4 decimal places for clean output
    let confidence = (best_score * 10000.0).round() / 10000.0;
    (&products[best_idx], confidence)
}
