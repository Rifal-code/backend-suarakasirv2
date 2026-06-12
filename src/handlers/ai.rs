use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    dto::{
        ai::{AiChatRequest, ParseOrderRequest},
        ApiResponse,
    },
    errors::AppError,
    middleware::Claims,
    repositories::ProductRepository,
    services::AiService,
    state::AppState,
};

pub async fn chat(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<AiChatRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let ai_service = AiService::new(
        state.config.ai_api_key.clone(),
        state.config.ai_api_url.clone(),
    );

    let response = ai_service.chat(req).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("AI response generated", response)),
    ))
}

/// POST /api/ai/parse-order
///
/// Menerima JSON hasil konversi Gemini dari suara ke teks:
///   { "items": [{ "n": "baxo", "q": 3 }] }
///
/// Melakukan fuzzy matching ke produk user di database,
/// mengembalikan daftar produk yang paling cocok dengan confidence score.
pub async fn parse_order(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<ParseOrderRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let product_repo = ProductRepository::new(state.db.clone());
    let ai_service = AiService::new(
        state.config.ai_api_key.clone(),
        state.config.ai_api_url.clone(),
    );

    let response = ai_service.parse_order(req, &product_repo, &claims.sub).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Order parsed from voice input", response)),
    ))
}
