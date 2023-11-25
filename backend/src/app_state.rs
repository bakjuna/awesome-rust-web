use std::sync::Arc;
use axum::extract::FromRef;

use shaku::module;

use crate::{DatabaseConnectionPool, DBConnection, health::{repository::RepositoryImpl, service::ServiceImpl}};


module! {
	pub ExampleModule {
			components = [DatabaseConnectionPool],
			providers = [DBConnection, RepositoryImpl, ServiceImpl]
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
