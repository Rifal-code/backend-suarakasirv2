use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::{handlers::feedback, middleware::jwt_middleware, state::AppState};

pub fn feedback_routes(state: AppState) -> Router<AppState> {
    // Public routes (no JWT required)
    let public_routes = Router::new()
        .route("/", get(feedback::list))
        .route("/{id}", get(feedback::show));

    // Protected routes (JWT required)
    let protected_routes = Router::new()
        .route("/", post(feedback::create))
        .route(
            "/{id}",
            put(feedback::update).delete(feedback::delete),
        )
        .route_layer(middleware::from_fn_with_state(state, jwt_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
}
