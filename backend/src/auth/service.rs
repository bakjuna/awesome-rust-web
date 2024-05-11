use axum::async_trait;
use shaku::Provider;
use crate::auth::repository::AuthRepository;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn get_double(&self) -> usize;
}

#[derive(Provider)]
#[shaku(interface = AuthService)]
pub struct AuthServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn AuthRepository>,
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn get_double(&self) -> usize {
        self.repo.get().await.test * 2
    }
}
