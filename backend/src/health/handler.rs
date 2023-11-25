use axum::response::{IntoResponse, Html};

pub async fn handler_health() -> impl IntoResponse {
	println!(" ->> {:<12} - handler-health", "GET");
	Html("OK".to_string())
}