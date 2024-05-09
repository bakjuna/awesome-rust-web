use futures::executor::block_on;
use shaku::{Component, HasComponent, Interface, Module, Provider};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::error::Error;
use std::sync::{Arc, RwLock};

pub trait ConnectionPool: Interface {
    fn get(&self) -> DBConnection;
}
fn create_db_pool() -> Pool<Postgres> {
    let database_url = format!("postgres://yacho:password@127.0.0.1:17342/public?schema=public");
    println!("Connecting Database..., {:?}", database_url);
    let pool = block_on(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url),
    )
    .unwrap();
    pool
}
#[derive(Component)]
#[shaku(interface = ConnectionPool)]
pub struct DatabaseConnectionPool {
    #[shaku(default=create_db_pool())]
    db: Pool<Postgres>,
}

impl ConnectionPool for DatabaseConnectionPool {
    fn get(&self) -> DBConnection {
        DBConnection(RwLock::new(self.db.clone()))
    }
}

pub struct DBConnection(pub RwLock<Pool<Postgres>>);

impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
    type Interface = DBConnection;

    fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error>> {
        let pool = module.resolve_ref().get();
        Ok(Box::new(pool))
    }
}
