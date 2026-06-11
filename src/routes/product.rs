use axum::{middleware, routing::get, Router};

use crate::{handlers::products, middleware::jwt_middleware, state::AppState};

pub fn product_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(products::list).post(products::create))
        .route(
            "/{id}",
            get(products::show)
                .put(products::update)
                .delete(products::delete),
        )
        .route_layer(middleware::from_fn_with_state(state, jwt_middleware))
}
