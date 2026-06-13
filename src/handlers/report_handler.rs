use axum::{
    body::Body,
    extract::{Extension, Query, State},
    http::{header, StatusCode},
    response::Response,
};

use crate::{
    dto::report::ReportRangeQuery,
    errors::AppError,
    middleware::Claims,
    repositories::UserRepository,
    services::ReportService,
    state::AppState,
};

/// GET /api/reports/sales/pdf?range=7d|30d|1y
///
/// Returns a PDF file (application/pdf) with the sales report
/// for the authenticated user's data within the given period.
pub async fn sales_pdf(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<ReportRangeQuery>,
) -> Result<Response<Body>, AppError> {
    // Load authenticated user (needed for UMKM info)
    let user_repo = UserRepository::new(state.db.clone());
    let user = user_repo
        .find_by_id(&claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Resolve font directory relative to current working dir
    let font_dir = std::env::var("FONT_DIR").unwrap_or_else(|_| "./fonts".to_string());

    let report_service = ReportService::new(
        state.db.clone(),
        state.config.ai_api_key.clone(),
        state.config.ai_api_url.clone(),
    );

    let pdf_bytes = report_service.generate_pdf(&user, &query, &font_dir).await?;

    let range = query.range.as_deref().unwrap_or("7d");
    let filename = format!("laporan-penjualan-{}.pdf", range);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .map_err(|e| AppError::InternalServerError(format!("Failed to build response: {}", e)))?;

    Ok(response)
}
