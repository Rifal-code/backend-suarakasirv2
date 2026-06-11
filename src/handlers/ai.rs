use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    dto::{ai::AiChatRequest, ApiResponse},
    errors::AppError,
    middleware::Claims,
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
