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
        let pool = self.db.0.lock().await.to_owned();
        let row: (i32,) = sqlx::query_as(
            r#"
                SELECT 1234;
            "#
        ).fetch_one(&pool).await.unwrap();
        let u = usize::try_from(row.0).unwrap();
        Test { test: u }
    }
}
