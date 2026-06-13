pub mod ai;
pub mod auth;
pub mod dashboard;
pub mod feedback;
pub mod order;
pub mod product;
pub mod report;

use axum::Router;

use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/auth", auth::auth_routes(state.clone()))
        .nest("/api/products", product::product_routes(state.clone()))
        .nest("/api/orders", order::order_routes(state.clone()))
        .nest("/api/feedback", feedback::feedback_routes(state.clone()))
        .nest("/api/ai", ai::ai_routes(state.clone()))
        .nest("/api/dashboard", dashboard::dashboard_routes(state.clone()))
        .nest("/api/reports", report::report_routes(state.clone()))
        .with_state(state)
}
