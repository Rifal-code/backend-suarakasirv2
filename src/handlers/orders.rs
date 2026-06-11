use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    dto::{
        order::{CreateOrderRequest, OrderListQuery, UpdateOrderRequest},
        ApiResponse, PaginatedResponse,
    },
    errors::AppError,
    middleware::Claims,
    repositories::{OrderRepository, ProductRepository},
    services::OrderService,
    state::AppState,
};

pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<OrderListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let order_repo = OrderRepository::new(state.db.clone());
    let product_repo = ProductRepository::new(state.db.clone());
    let service = OrderService::new(order_repo, product_repo);

    let (orders, total, page, limit) = service.list(&claims.sub, query).await?;

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::success(
            "Orders fetched successfully",
            orders,
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
    let order_repo = OrderRepository::new(state.db.clone());
    let product_repo = ProductRepository::new(state.db.clone());
    let service = OrderService::new(order_repo, product_repo);

    let order = service.get(&id, &claims.sub).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Order fetched successfully", order)),
    ))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let order_repo = OrderRepository::new(state.db.clone());
    let product_repo = ProductRepository::new(state.db.clone());
    let service = OrderService::new(order_repo, product_repo);

    let order = service.create(&claims.sub, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Order created successfully", order)),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(req): Json<UpdateOrderRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let order_repo = OrderRepository::new(state.db.clone());
    let product_repo = ProductRepository::new(state.db.clone());
    let service = OrderService::new(order_repo, product_repo);

    let order = service.update(&id, &claims.sub, req).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Order updated successfully", order)),
    ))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let order_repo = OrderRepository::new(state.db.clone());
    let product_repo = ProductRepository::new(state.db.clone());
    let service = OrderService::new(order_repo, product_repo);

    service.delete(&id, &claims.sub).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Order deleted successfully",
            serde_json::json!(null),
        )),
    ))
}
