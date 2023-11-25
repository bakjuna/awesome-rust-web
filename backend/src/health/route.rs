use axum::{Router, routing::get};

use crate::AppState;
use crate::health::handler::handler_health;

pub fn router_health(state: AppState) -> Router {
    Router::new().route("/", get(handler_health))
    .with_state(state)
}