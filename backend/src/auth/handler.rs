use axum::Json;
use shaku_axum::InjectProvided;

use crate::AppModule;
// use crate::AppModule;

use super::{model::Auth, service::AuthService};
// use super::service::AuthService;
pub async fn handler_auth(
    hello_world: InjectProvided<AppModule, dyn AuthService>,
) -> Json<Auth> {
    let auth: Auth = Auth {
        is_ok: hello_world.get_double().await,
    };
    let res: Json<Auth> = Json(auth);
    res
}
