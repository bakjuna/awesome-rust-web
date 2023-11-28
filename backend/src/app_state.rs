use axum::extract::FromRef;
use std::sync::Arc;

use crate::{
    database::{DBConnection, DatabaseConnectionPool},
    health::repository::HealthRepositoryImpl,
    health::service::HealthServiceImpl,
    env::EnvImpl,
};
use shaku::module;
module! {
    pub AppModule {
        components = [DatabaseConnectionPool, EnvImpl],
        providers = [DBConnection, HealthServiceImpl, HealthRepositoryImpl]
    }
}
#[derive(Clone)]
pub struct AppState {
    pub(crate) module: Arc<AppModule>,
}

impl FromRef<AppState> for Arc<AppModule> {
    fn from_ref(app_state: &AppState) -> Arc<AppModule> {
        app_state.module.clone()
    }
}
