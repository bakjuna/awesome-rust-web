use crate::{
    auth::{repository::AuthRepositoryImpl, service::AuthServiceImpl},
    cron::component::CronJobComponent,
    database::{DatabaseConnectionPool, PoolProvider},
    env::{EnvComponent, EnvProvider},
    health::{repository::HealthRepositoryImpl, service::HealthServiceImpl}, kafka::{adaptor::KafkaAdaptorImpl, component::{KafkaComponent, KafkaProducerProvider}},
};
use axum::extract::FromRef;
use shaku::module;
use std::sync::Arc;

module! {
    pub AppModule {
        components = [CronJobComponent, KafkaComponent, DatabaseConnectionPool, EnvComponent],
        providers = [EnvProvider,PoolProvider,HealthServiceImpl, HealthRepositoryImpl,AuthServiceImpl,AuthRepositoryImpl, KafkaProducerProvider, KafkaAdaptorImpl]
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
