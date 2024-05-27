use crate::database::PoolProvider;
use crate::errors::CustomError;
use axum::async_trait;
use shaku::Provider;

use super::model::Auth;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn get(&self) -> Result<Auth, CustomError>;
}

#[derive(Provider)]
#[shaku(interface = AuthRepository)]
pub struct AuthRepositoryImpl {
    #[shaku(provide)]
    db: Box<PoolProvider>,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn get(&self) -> Result<Auth, CustomError> {
        sqlx::query_as::<_, (i32,)>(
            r#"
                SELECT 1234;
            "#,
        )
        .fetch_one(self.db.0.as_ref())
        .await
        .map_err(|_e| CustomError::NotFoundError)
        .and_then(|row| {
            usize::try_from(row.0)
                .map_err(|_e| CustomError::NumberParsingError)
                .map(|u| Auth { is_ok: u })
        })
    }
}
