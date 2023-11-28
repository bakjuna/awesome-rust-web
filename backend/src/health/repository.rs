use crate::database::DBConnection;
use crate::health::model::Test;
use axum::async_trait;
use shaku::Provider;

#[async_trait]
pub trait HealthRepository: Send + Sync {
    async fn get(&self) -> Test;
}

#[derive(Provider)]
#[shaku(interface = HealthRepository)]
pub struct HealthRepositoryImpl {
    #[shaku(provide)]
    db: Box<DBConnection>,
}

#[async_trait]
impl HealthRepository for HealthRepositoryImpl {
    async fn get(&self) -> Test {
        let _p = (self.db.0).lock().await;
        Test { test: 1 }
    }
}
