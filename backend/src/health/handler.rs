use axum::{response::Json as JsonResponse, extract::Json};
use shaku_axum::InjectProvided;
use crate::{errors::CustomError, app_state::AppModule};

use super::{model::Health, service::HealthService};
pub async fn handler_health(
    hello_world: InjectProvided<AppModule, dyn HealthService>,
) -> Result<Json<Health>, CustomError> {
    match hello_world.get_double().await {
        Ok(res) => Ok(JsonResponse(Health { is_ok: res })),
        Err(e) => Err(e)
    }

}
