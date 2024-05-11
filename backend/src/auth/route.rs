use axum::{routing::get, Router};

use crate::auth::handler::handler_auth;
use crate::AppState;

pub fn router_auth(state: AppState) -> Router {
    let method_router = get(handler_auth);
    Router::new().route("/", method_router).with_state(state)
}
