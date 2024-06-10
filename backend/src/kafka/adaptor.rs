use std::sync::Arc;

use crate::database::PoolProvider;
use crate::errors::CustomError;
use axum::async_trait;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;
use shaku::Provider;

use super::component::KafkaProducerProvider;

#[async_trait]
pub trait KafkaAdaptor: Send + Sync {
    async fn send_topic_1(&self) -> Result<(), CustomError>;
		async fn send_topic_2(&self) -> Result<(), CustomError>;
}

#[derive(Provider)]
#[shaku(interface = KafkaAdaptor)]
pub struct KafkaAdaptorImpl {
    #[shaku(provide)]
    db: Box<PoolProvider>,
    #[shaku(provide)]
    kafka_producer: Box<KafkaProducerProvider>
}

#[async_trait]
impl KafkaAdaptor for KafkaAdaptorImpl {
    async fn send_topic_1(&self) -> Result<(), CustomError> {
			let k = Arc::clone(&self.kafka_producer.0);
			println!("kafka producer here");
			k.send(FutureRecord::<(), _>::to("chat")
			.payload("hello here"), Timeout::Never).await.unwrap();
			Ok(())
    }
		async fn send_topic_2(&self) -> Result<(), CustomError> {
			let k = Arc::clone(&self.kafka_producer.0);
			println!("kafka producer here");
			k.send(FutureRecord::<(), _>::to("pong")
			.payload("hello here for pong"), Timeout::Never).await.unwrap();
			Ok(())
    }
}
