use std::borrow::Borrow;

use crate::database::PoolProvider;
use crate::env::EnvProvider;
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
    db: Box<PoolProvider>,
    #[shaku(provide)]
    env: Box<EnvProvider>,
}

#[async_trait]
impl HealthRepository for HealthRepositoryImpl {
    async fn get(&self) -> Test {
        let pool = self.db.0.borrow();
        let row: (i32,) = sqlx::query_as(
            r#"
                SELECT 1234;
            "#
        ).fetch_one(pool).await.unwrap();
        println!("env: {:?}", self.env.server.address);
        let u = usize::try_from(row.0).unwrap();
        Test { test: u }
    }
}
