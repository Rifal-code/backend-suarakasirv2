use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    dto::{
        auth::{LoginRequest, RegisterRequest, UpdateProfileRequest},
        ApiResponse,
    },
    errors::AppError,
    middleware::Claims,
    repositories::UserRepository,
    services::AuthService,
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user_repo = UserRepository::new(state.db.clone());
    let auth_service = AuthService::new(user_repo, state.config.jwt_secret.clone());

    let response = auth_service.register(req).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Registration successful", response)),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user_repo = UserRepository::new(state.db.clone());
    let auth_service = AuthService::new(user_repo, state.config.jwt_secret.clone());

    let response = auth_service.login(req).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Login successful", response)),
    ))
}

pub async fn logout(
    Extension(_claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Logout successful. Please discard your token.",
            serde_json::json!(null),
        )),
    ))
}

pub async fn get_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let user_repo = UserRepository::new(state.db.clone());
    let auth_service = AuthService::new(user_repo, state.config.jwt_secret.clone());

    let profile = auth_service.get_profile(&claims.sub).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Profile fetched successfully", profile)),
    ))
}

pub async fn update_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user_repo = UserRepository::new(state.db.clone());
    let auth_service = AuthService::new(user_repo, state.config.jwt_secret.clone());

    let profile = auth_service.update_profile(&claims.sub, req).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Profile updated successfully", profile)),
    ))
}
