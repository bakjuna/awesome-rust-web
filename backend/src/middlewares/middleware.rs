use axum::{http::{Uri, Method}, response::{Response, IntoResponse}, Json};
use serde_json::json;
use uuid::Uuid;

use crate::{log_request, errors::Error};


pub async fn main_response_mapper(
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	let service_error = res.extensions().get::<Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

				println!("    ->> client_error_body: {client_error_body}");

				(*status_code, Json(client_error_body)).into_response()
			});

	let client_error = client_status_error.unzip().1;
	let _ =
		log_request(uuid, req_method, uri, service_error, client_error).await;
	error_response.unwrap_or(res)
}