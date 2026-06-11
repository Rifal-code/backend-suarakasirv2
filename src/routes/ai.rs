use axum::{
    middleware,
    routing::post,
    Router,
};

use crate::{handlers::ai, middleware::jwt_middleware, state::AppState};

pub fn ai_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/chat", post(ai::chat))
        .route_layer(middleware::from_fn_with_state(state, jwt_middleware))
}
