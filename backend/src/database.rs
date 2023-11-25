pub trait ConnectionPool: Interface {
	fn get(&self) -> DBConnection;
}

pub struct DBConnection(RefCell<usize>);

#[derive(Component)]
#[shaku(interface = ConnectionPool)]
pub struct DatabaseConnectionPool {
    #[shaku(default = 42)]
    value: usize,
}

impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
	type Interface = DBConnection;


	fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
			let pool: &dyn ConnectionPool = module.resolve_ref();
			Ok(Box::new(pool.get()))
	}
}

impl ConnectionPool for DatabaseConnectionPool {
    fn get(&self) -> DBConnection {
        DBConnection(RefCell::new(self.value))
    }
}