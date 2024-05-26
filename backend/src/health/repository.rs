use std::borrow::Borrow;

use crate::database::PoolProvider;
use crate::env::EnvProvider;
use crate::errors::CustomError;
use crate::health::model::Test;
use axum::async_trait;
use shaku::Provider;

#[async_trait]
pub trait HealthRepository: Send + Sync {
    async fn get(&self) -> Result<Test, CustomError>;
}

#[derive(Provider)]
#[shaku(interface = HealthRepository)]
pub struct HealthRepositoryImpl {
    #[shaku(provide)]
    db: Box<PoolProvider>,
    #[shaku(provide)]
    env: Box<EnvProvider>,
}

#[async_trait]
impl HealthRepository for HealthRepositoryImpl {
    async fn get(&self) -> Result<Test, CustomError> {
        sqlx::query_as::<_, (i32,)>(
            r#"
                SELECT 1111;
            "#,
        )
        .fetch_one(self.db.0.as_ref())
        .await
        .map_err(|_e| CustomError::NotFoundError)
        .and_then(|row| {
            usize::try_from(row.0)
                .map_err(|_e| CustomError::NumberParsingError)
                .map(|u| Test { test: u })
        })
    }
}
