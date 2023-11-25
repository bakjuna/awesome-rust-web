
use axum::{Json};
use shaku_axum::{Inject, InjectProvided};

use crate::{Result, ExampleModule};
// use crate::AppModule;

use super::{model::Health, service::Service};
// use super::service::HealthService;
pub async fn handler_health(data: InjectProvided<ExampleModule, dyn Service>) -> Json<Health> {
    println!(" ->> {:<12} - handler-health", "GET");
		let p = data.get_double();
    let health: Health = Health { is_ok: p };
		let res: Json<Health> = Json(health);
    res
}
