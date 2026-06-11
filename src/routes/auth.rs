use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{handlers::auth, middleware::jwt_middleware, state::AppState};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    let protected_routes = Router::new()
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::get_profile).put(auth::update_profile))
        .route_layer(middleware::from_fn_with_state(state, jwt_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
}