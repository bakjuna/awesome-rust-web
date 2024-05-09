use axum::extract::FromRef;
use std::sync::Arc;

use crate::{
    database::{PoolProvider, DatabaseConnectionPool}, env::{ EnvComponent, EnvProvider}, health::{repository::HealthRepositoryImpl, service::HealthServiceImpl}
};
use shaku::module;
module! {
    pub AppModule {
        components = [DatabaseConnectionPool, EnvComponent],
        providers = [EnvProvider, PoolProvider, HealthServiceImpl, HealthRepositoryImpl]
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
