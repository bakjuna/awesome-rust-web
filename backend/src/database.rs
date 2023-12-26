use futures::executor::block_on;
use shaku::{Component, HasComponent, Interface, Module, Provider};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub trait ConnectionPool: Interface {
    fn initialize(&self) -> DBConnection;
}

#[derive(Component)]
#[shaku(interface = ConnectionPool)]
pub struct DatabaseConnectionPool {
}

impl ConnectionPool for DatabaseConnectionPool {
    fn initialize(&self) -> DBConnection {
        let database_url =
            format!("postgres://yacho:password@127.0.0.1:17342/public?schema=public");
        println!("Connecting Database..., {:?}", database_url);
        let pool = block_on(
            PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url),
        )
            .unwrap();
        DBConnection(Arc::new(Mutex::new(pool)))
    }
}

pub struct DBConnection(pub Arc<Mutex<Pool<Postgres>>>);


impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
    type Interface = DBConnection;

    fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error>> {
        let pool = module.resolve_ref().initialize();
        Ok(Box::new(pool))
    }
}


