use crate::{
    auth::{repository::AuthRepositoryImpl, service::AuthServiceImpl},
    cron::component::CronJobComponent,
    database::{DatabaseConnectionPool, PoolProvider},
    env::{EnvComponent, EnvProvider},
    health::{repository::HealthRepositoryImpl, service::HealthServiceImpl},
};
use axum::extract::FromRef;
use shaku::module;
use std::sync::Arc;

module! {
    pub AppModule {
        components = [CronJobComponent, DatabaseConnectionPool, EnvComponent],
        providers = [EnvProvider,PoolProvider,HealthServiceImpl, HealthRepositoryImpl,AuthServiceImpl,AuthRepositoryImpl,]
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
