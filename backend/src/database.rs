use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
use std::cell::RefCell;
use std::error::Error;

// Traits

pub trait ConnectionPool: Interface {
    fn get(&self) -> DBConnection;
}

// Structs

pub struct DBConnection(pub usize);

#[derive(Component)]
#[shaku(interface = ConnectionPool)]
pub struct DatabaseConnectionPool {
    #[shaku(default = 42)]
    pub value: usize,
}


// Trait implementations


impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
    type Interface = DBConnection;

    fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
        let pool: &dyn ConnectionPool = module.resolve_ref();
        Ok(Box::new(pool.get()))
    }
}

impl ConnectionPool for DatabaseConnectionPool {
    fn get(&self) -> DBConnection {
        DBConnection(self.value)
    }
}
