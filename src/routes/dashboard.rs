use axum::{middleware, routing::get, Router};

use crate::{handlers::dashboard, middleware::jwt_middleware, state::AppState};

pub fn dashboard_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(dashboard::overview))
        .route("/sales", get(dashboard::sales_chart))
        .route("/top-products", get(dashboard::top_products))
        .route("/trends", get(dashboard::trends))
        .route_layer(middleware::from_fn_with_state(state, jwt_middleware))
}
