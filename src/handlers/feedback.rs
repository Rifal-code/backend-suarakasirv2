use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    dto::{
        feedback::{CreateFeedbackRequest, FeedbackListQuery, UpdateFeedbackRequest},
        ApiResponse, PaginatedResponse,
    },
    errors::AppError,
    middleware::Claims,
    repositories::FeedbackRepository,
    services::FeedbackService,
    state::AppState,
};

pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<FeedbackListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let feedback_repo = FeedbackRepository::new(state.db.clone());
    let service = FeedbackService::new(feedback_repo);

    let (feedbacks, total, page, limit) = service.list(query).await?;

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::success(
            "Feedback fetched successfully",
            feedbacks,
            total,
            page,
            limit,
        )),
    ))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let feedback_repo = FeedbackRepository::new(state.db.clone());
    let service = FeedbackService::new(feedback_repo);

    let feedback = service.get(&id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Feedback fetched successfully", feedback)),
    ))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateFeedbackRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let feedback_repo = FeedbackRepository::new(state.db.clone());
    let service = FeedbackService::new(feedback_repo);

    let feedback = service.create(&claims.sub, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Feedback created successfully", feedback)),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(req): Json<UpdateFeedbackRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let feedback_repo = FeedbackRepository::new(state.db.clone());
    let service = FeedbackService::new(feedback_repo);

    let feedback = service.update(&id, &claims.sub, req).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Feedback updated successfully", feedback)),
    ))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let feedback_repo = FeedbackRepository::new(state.db.clone());
    let service = FeedbackService::new(feedback_repo);

    service.delete(&id, &claims.sub).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Feedback deleted successfully",
            serde_json::json!(null),
        )),
    ))
}
