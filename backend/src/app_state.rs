use std::sync::Arc;
use axum::extract::FromRef;

use shaku::module;

use crate::{ health::{repository::RepositoryImpl, service::ServiceImpl}, env::EnvImpl, PgPoolImpl, DBConnection};


module! {
	pub ExampleModule {
			components = [DBConnection, EnvImpl],
			providers = [ RepositoryImpl, ServiceImpl]
	}
}

impl FromRef<AppState> for Arc<ExampleModule> {
	fn from_ref(app_state: &AppState) -> Arc<ExampleModule> {
			app_state.module.clone()
	}
}

#[derive(Clone)]
pub struct AppState {
    pub module: Arc<ExampleModule>,
}
