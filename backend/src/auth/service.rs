use std::sync::Arc;

use axum::async_trait;
use rdkafka::{producer::FutureRecord, util::Timeout};
use shaku::Provider;
use crate::{auth::repository::AuthRepository, errors::CustomError, kafka::{adaptor::KafkaAdaptor, component::KafkaProducerProvider}};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn get_double(&self, input_number: usize) -> Result<usize, CustomError>;
}

#[derive(Provider)]
#[shaku(interface = AuthService)]
pub struct AuthServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn AuthRepository>,
    #[shaku(provide)]
    kafka_adaptor: Box<dyn KafkaAdaptor>
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn get_double(&self, input_number: usize) -> Result<usize, CustomError> {
        self.kafka_adaptor.send_topic_1().await.unwrap();
        match self.repo.get().await {
            Ok(n) => Ok(n.is_ok * input_number),
            Err(e) => Err(e)
        }
    }
}
