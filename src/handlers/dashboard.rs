use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    dto::{dashboard::DashboardRangeQuery, ApiResponse},
    errors::AppError,
    middleware::Claims,
    repositories::DashboardRepository,
    services::DashboardService,
    state::AppState,
};

pub async fn overview(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let repo = DashboardRepository::new(state.db.clone());
    let service = DashboardService::new(repo);

    let data = service.overview(&claims.sub).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Dashboard overview", data)),
    ))
}

pub async fn sales_chart(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<DashboardRangeQuery>,
) -> Result<impl IntoResponse, AppError> {
    let repo = DashboardRepository::new(state.db.clone());
    let service = DashboardService::new(repo);

    let data = service.sales_chart(&claims.sub, &query).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Sales chart data", data)),
    ))
}

pub async fn top_products(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<DashboardRangeQuery>,
) -> Result<impl IntoResponse, AppError> {
    let repo = DashboardRepository::new(state.db.clone());
    let service = DashboardService::new(repo);

    let data = service.top_products(&claims.sub, &query).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Top products", data)),
    ))
}

pub async fn trends(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<DashboardRangeQuery>,
) -> Result<impl IntoResponse, AppError> {
    let repo = DashboardRepository::new(state.db.clone());
    let service = DashboardService::new(repo);

    let data = service.trends(&claims.sub, &query).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Sales trends", data)),
    ))
}
