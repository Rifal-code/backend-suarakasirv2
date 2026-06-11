use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    dto::{
        product::{CreateProductRequest, ProductListQuery, UpdateProductRequest},
        ApiResponse, PaginatedResponse,
    },
    errors::AppError,
    middleware::Claims,
    repositories::ProductRepository,
    services::ProductService,
    state::AppState,
};

pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<ProductListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let product_repo = ProductRepository::new(state.db.clone());
    let service = ProductService::new(product_repo);

    let (products, total, page, limit) = service.list(&claims.sub, query).await?;

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::success(
            "Products fetched successfully",
            products,
            total,
            page,
            limit,
        )),
    ))
}

pub async fn show(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let product_repo = ProductRepository::new(state.db.clone());
    let service = ProductService::new(product_repo);

    let product = service.get(&id, &claims.sub).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Product fetched successfully", product)),
    ))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let product_repo = ProductRepository::new(state.db.clone());
    let service = ProductService::new(product_repo);

    let product = service.create(&claims.sub, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Product created successfully", product)),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let product_repo = ProductRepository::new(state.db.clone());
    let service = ProductService::new(product_repo);

    let product = service.update(&id, &claims.sub, req).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Product updated successfully", product)),
    ))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let product_repo = ProductRepository::new(state.db.clone());
    let service = ProductService::new(product_repo);

    service.delete(&id, &claims.sub).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Product deleted successfully",
            serde_json::json!(null),
        )),
    ))
}
