use axum::{Router, routing::get};

pub fn router_health() -> Router {
    Router::new().route("/", get(super::handler::handler_health))
}
