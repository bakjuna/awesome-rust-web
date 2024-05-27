use axum::{response::Json as JsonResponse, extract::Json};
use shaku_axum::InjectProvided;

use crate::{errors::CustomError, AppModule};
// use crate::AppModule;

use super::{model::Auth, service::AuthService};
// use super::service::AuthService;
pub async fn handler_auth(
    hello_world: InjectProvided<AppModule, dyn AuthService>,
    Json(payload): Json<usize>,
) -> Result<JsonResponse<Auth>, CustomError> {
    match hello_world.get_double(payload).await {
        Ok(res) => Ok(JsonResponse(Auth { is_ok: res })),
        Err(e) => Err(e),
    }
}
