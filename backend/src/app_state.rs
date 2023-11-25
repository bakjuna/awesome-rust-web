use std::sync::Arc;

use shaku::module;



module! {
	pub AppModule {
			components = [HealthServiceImpl],
			providers = []
	}
}

#[derive(Clone)]
pub struct AppState {
    pub module: Arc<AppModule>,
}

impl FromRef<AppState> for Arc<AppModule> {
    fn from_ref(app_state: &AppState) -> Arc<AppModule> {
        app_state.module.clone()
    }
}