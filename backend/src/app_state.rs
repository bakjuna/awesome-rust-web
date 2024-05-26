use axum::extract::FromRef;
use std::sync::Arc;

use crate::{
    auth::{repository::AuthRepositoryImpl, service::AuthServiceImpl},
    database::{DatabaseConnectionPool, PoolProvider},
    env::{EnvComponent, EnvProvider},
    health::{repository::HealthRepositoryImpl, service::HealthServiceImpl},
};
use shaku::module;
module! {
    pub AppModule {
        components = [DatabaseConnectionPool, EnvComponent],
        providers = [EnvProvider, PoolProvider, HealthServiceImpl, HealthRepositoryImpl, AuthServiceImpl, AuthRepositoryImpl]
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
