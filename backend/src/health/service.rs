use axum::async_trait;
use shaku::Provider;
use crate::{errors::CustomError, health::repository::HealthRepository};

#[async_trait]
pub trait HealthService: Send + Sync {
    async fn get_double(&self) -> Result<usize, CustomError>;
}

#[derive(Provider)]
#[shaku(interface = HealthService)]
pub struct HealthServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn HealthRepository>,
}

#[async_trait]
impl HealthService for HealthServiceImpl {
    async fn get_double(&self) -> Result<usize, CustomError> {
        match self.repo.get().await {
            Ok(res) => Ok(res.test * 2),
            Err(e) => Err(e),
        }
    }
}
