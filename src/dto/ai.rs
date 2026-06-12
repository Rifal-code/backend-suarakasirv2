use serde::{Deserialize, Serialize};
use validator::Validate;

// ─────────────────────────────────────────────
// General AI chat
// ─────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct AiChatRequest {
    #[validate(length(min = 1, max = 2000, message = "Message must be between 1 and 2000 characters"))]
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AiChatResponse {
    pub reply: String,
}

// ─────────────────────────────────────────────
// Voice / AI parse-order
// ─────────────────────────────────────────────

/// Raw input item from Gemini voice-to-JSON conversion.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawOrderItem {
    /// Nama produk yang diucapkan (bisa typo/pendek)
    pub n: String,
    /// Jumlah produk
    pub q: i32,
}

/// Request body untuk POST /ai/parse-order
#[derive(Debug, Deserialize, Validate)]
pub struct ParseOrderRequest {
    #[validate(length(min = 1, message = "items tidak boleh kosong"))]
    pub items: Vec<RawOrderItem>,
}

/// Hasil fuzzy match satu item
#[derive(Debug, Serialize)]
pub struct MatchedOrderItem {
    pub product_id: String,
    pub name: String,
    pub input_name: String,
    pub quantity: i32,
    pub unit_price: rust_decimal::Decimal,
    pub confidence: f64,
    /// true jika confidence di bawah threshold, perlu konfirmasi user
    pub needs_confirmation: bool,
}

/// Response body untuk POST /ai/parse-order
#[derive(Debug, Serialize)]
pub struct ParseOrderResponse {
    pub items: Vec<MatchedOrderItem>,
}
