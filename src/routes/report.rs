use axum::{middleware, routing::get, Router};

use crate::{handlers::report_handler, middleware::jwt_middleware, state::AppState};

pub fn report_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/sales/pdf", get(report_handler::sales_pdf))
        .route_layer(middleware::from_fn_with_state(state, jwt_middleware))
}
