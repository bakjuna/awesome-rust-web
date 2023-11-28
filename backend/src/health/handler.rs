use axum::Json;
use shaku_axum::InjectProvided;

use crate::AppModule;
// use crate::AppModule;

use super::{model::Health, service::HealthService};
// use super::service::HealthService;
pub async fn handler_health(
    hello_world: InjectProvided<AppModule, dyn HealthService>,
) -> Json<Health> {
    let health: Health = Health {
        is_ok: hello_world.get_double().await,
    };
    let res: Json<Health> = Json(health);
    res
}
