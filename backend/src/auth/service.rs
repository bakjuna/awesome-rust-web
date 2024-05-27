use axum::async_trait;
use shaku::Provider;
use crate::{auth::repository::AuthRepository, errors::CustomError};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn get_double(&self, input_number: usize) -> Result<usize, CustomError>;
}

#[derive(Provider)]
#[shaku(interface = AuthService)]
pub struct AuthServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn AuthRepository>,
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn get_double(&self, input_number: usize) -> Result<usize, CustomError> {
        match self.repo.get().await {
            Ok(n) => Ok(n.is_ok * input_number),
            Err(e) => Err(e)
        }
    }
}
