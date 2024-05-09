use crate::env::create_env;
use futures::executor::block_on;
use shaku::{Component, HasComponent, Interface, Module, Provider};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::error::Error;

pub trait ConnectionPool: Interface {
    fn get(&self) -> PoolProvider;
}
fn create_db_pool() -> Pool<Postgres> {
    let postgres_settings = create_env().postgres;
    let database_url = format!(
        "postgres://{user}:{password}@{host}:{port}/{database}?schema={schema}",
        user = postgres_settings.user,
        password = postgres_settings.password,
        host = postgres_settings.host,
        port = postgres_settings.port,
        database = postgres_settings.database,
        schema = postgres_settings.schema
    );
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
    fn get(&self) -> PoolProvider {
        PoolProvider(self.db.clone())
    }
}

pub struct PoolProvider(pub Pool<Postgres>);

impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for PoolProvider {
    type Interface = PoolProvider;

    fn provide(module: &M) -> Result<Box<PoolProvider>, Box<dyn Error>> {
        let pool = module.resolve_ref().get();
        Ok(Box::new(pool))
    }
}
