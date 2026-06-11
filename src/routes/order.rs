use axum::{middleware, routing::get, Router};

use crate::{handlers::orders, middleware::jwt_middleware, state::AppState};

pub fn order_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(orders::list).post(orders::create))
        .route(
            "/{id}",
            get(orders::show)
                .put(orders::update)
                .delete(orders::delete),
        )
        .route_layer(middleware::from_fn_with_state(state, jwt_middleware))
}
