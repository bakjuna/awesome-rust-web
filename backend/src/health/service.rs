use shaku::Provider;

pub trait HealthService {
	fn get_health(&self) -> String;
}


#[derive(Provider)]
#[shaku(interface = HealthService)]
struct HealthServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn HealthRepository>
}


impl HealthService for HealthServiceImpl {
	fn get_health(&self) -> String {
		"Hello, world!".to_owned()
	}
}
