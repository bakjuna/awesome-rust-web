use axum::extract::FromRef;
use shaku::{module, Component, HasProvider, Interface, Module, Provider, HasComponent};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::error::Error;
use std::future::Future;
use std::sync::Arc;

use crate::env::Env;

// Define the `ConnectionPool` trait
pub trait ConnectionPool: Interface {
    // fn initialize(&self) -> Pool<Postgres>;
}

// Define the `DBConnection` struct
pub struct DBConnection(pub Box<dyn Future<Output= Pool<Postgres>>>);

// Define the `DatabaseConnectionPool` struct as a Shaku component
#[derive(Component)]
#[shaku(interface = ConnectionPool)]
pub struct DatabaseConnectionPool {
    pub value: usize,
    pub env: Arc<dyn Env>,
}

// Implement the `Provider` trait for `DatabaseConnectionPool`
impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
    type Interface = DBConnection;

    fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
        let database_url = format!(
            "postgres://yacho:password@127.0.0.1:17342/public?schema=public"
        );
        println!("Connecting Database..., {:?}", database_url);
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url);

        Ok(Box::new(DBConnection(pool)))
    }
}

// Implement the `ConnectionPool` trait for `DatabaseConnectionPool`
impl ConnectionPool for DatabaseConnectionPool {
    // fn initialize(&self) -> Pool<Postgres> {
    //     self.provide().unwrap().0
    // }
}