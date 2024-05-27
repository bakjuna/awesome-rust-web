use axum::{extract::Json, response::Json as JsonResponse};
use shaku_axum::InjectProvided;
use crate::{app_state::AppModule, errors::CustomError};
use super::{model::Auth, service::AuthService};

pub async fn handler_auth(
    hello_world: InjectProvided<AppModule, dyn AuthService>,
    Json(payload): Json<Auth>,
) -> Result<JsonResponse<Auth>, CustomError> {
    match hello_world.get_double(payload.is_ok).await {
        Ok(res) => Ok(JsonResponse(Auth { is_ok: res })),
        Err(e) => Err(e),
    }
}
