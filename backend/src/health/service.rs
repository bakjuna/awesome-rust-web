use axum::async_trait;
use shaku::Provider;

use crate::health::repository::HealthRepository;

#[async_trait]
pub trait HealthService: Send + Sync {
    async fn get_double(&self) -> usize;
}

#[derive(Provider)]
#[shaku(interface = HealthService)]
pub struct HealthServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn HealthRepository>,
}
#[async_trait]
impl HealthService for HealthServiceImpl {
    async fn get_double(&self) -> usize {
        self.repo.get().await.test * 2
    }
}
