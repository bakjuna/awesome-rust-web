use axum::{routing::get, Router};

use crate::health::handler::handler_health;
use crate::AppState;

pub fn router_health(state: AppState) -> Router {
    let method_router = get(handler_health);
    Router::new().route("/", method_router).with_state(state)
}
