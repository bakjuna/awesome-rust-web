use crate::{
    database::ConnectionPool,
    env::{create_env, Env},
};
use futures::StreamExt;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig, Message,
};
use shaku::{Component, HasComponent, Interface, Module, Provider};
use std::{error::Error, str::from_utf8, sync::Arc};
use uuid::Uuid;

pub trait KafkaInterface: Interface {
    fn initialize(self: Arc<Self>);
    fn get_producer(&self) -> KafkaProducerProvider;
    fn consumer_for_1(&self, payload: &str);
    fn consumer_for_2(&self, payload: &str);
}
fn create_producer() -> Arc<FutureProducer> {
    let env = create_env();
    Arc::new(
        ClientConfig::new()
            .set("bootstrap.servers", env.kafka.server)
            .set("queue.buffering.max.ms", "0")
            .create()
            .expect("Failed to create client"),
    )
}

fn create_consumer_1() -> Arc<StreamConsumer> {
    let env = create_env();
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", env.kafka.server)
        .set("enable.partition.eof", "false")
        // We'll give each session its own (unique) consumer group id,
        // so that each session will receive all messages
        .set("group.id", format!("chat-{}", Uuid::new_v4()))
        .create()
        .expect("Failed to create client");
    consumer.subscribe(&["chat"]).unwrap();
    Arc::new(consumer)
}
fn create_consumer_2() -> Arc<StreamConsumer> {
    let env = create_env();
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", env.kafka.server)
        .set("enable.partition.eof", "false")
        .set("group.id", format!("chat-{}", Uuid::new_v4()))
        .create()
        .expect("Failed to create client");
    consumer.subscribe(&["pong"]).unwrap();
    Arc::new(consumer)
}
#[derive(Component)]
#[shaku(interface = KafkaInterface)]
pub struct KafkaComponent {
    #[shaku(default=create_producer())]
    producer: Arc<FutureProducer>,
    #[shaku(default=create_consumer_1())]
    consumer_1: Arc<StreamConsumer>,
    #[shaku(default=create_consumer_2())]
    consumer_2: Arc<StreamConsumer>,
}

impl KafkaInterface for KafkaComponent {

    fn initialize(self: Arc<Self>) {
        futures::executor::block_on(async {
            let mut stream_1 = self.consumer_1.stream();
            let mut stream_2 = self.consumer_2.stream();
            loop {
                tokio::select! {
                    result = stream_1.next() => match result {
                        Some(Ok(msg)) => {
                            if let Some(payload) = msg.payload() {
                                match from_utf8(payload) {
                                    Ok(str_msg) => {
                                        self.consumer_for_1(str_msg);
                                    }
                                    Err(e) => println!("Error decoding message: {}", e),
                                }
                            } else {
                                println!("Consumer 1 received empty payload");
                            }
                        },
                        Some(Err(e)) => {
                            println!("Error receiving message in consumer 1: {}", e);
                            break;
                        },
                        None => break,
                    },
                    result = stream_2.next() => match result {
                        Some(Ok(msg)) => {
                            if let Some(payload) = msg.payload() {
                                match from_utf8(payload) {
                                    Ok(str_msg) => {
                                        self.consumer_for_2(str_msg);
                                    }
                                    Err(e) => println!("Error decoding message: {}", e),
                                }
                            } else {
                                println!("Consumer 2 received empty payload");
                            }
                        },
                        Some(Err(e)) => {
                            println!("Error receiving message in consumer 2: {}", e);
                            break;
                        },
                        None => break,
                    },
                }
            }
        });
    }

    fn consumer_for_1(&self, payload: &str) {
        println!("Consumer 1 received message: {:?}", payload);
    }

    fn consumer_for_2(&self, payload: &str) {
        println!("Consumer 2 received message: {:?}", payload);
    }
    fn get_producer(&self) -> KafkaProducerProvider {
        KafkaProducerProvider(Arc::clone(&self.producer))
    }
}

pub struct KafkaProducerProvider(pub Arc<FutureProducer>);

impl<M: Module + HasComponent<dyn KafkaInterface>> Provider<M> for KafkaProducerProvider {
    type Interface = KafkaProducerProvider;

    fn provide(module: &M) -> Result<Box<KafkaProducerProvider>, Box<dyn Error>> {
        let pool = module.resolve_ref().get_producer();
        Ok(Box::new(pool))
    }
}
