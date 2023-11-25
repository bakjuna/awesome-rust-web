use shaku::Provider;

pub trait HealthRepository {
	fn get(&self) -> usize;
}

#[derive(Provider)]
#[shaku(interface = HealthRepository)]
pub struct HealthRepositoryImpl {
    #[shaku(provide)]
    db: Box<DBConnection>
}

impl HealthRepository for HealthRepositoryImpl {
	fn get(&self) -> usize {
			*(*self.db).0.borrow()
	}
}