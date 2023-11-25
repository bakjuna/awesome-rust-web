use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
use std::cell::RefCell;
use std::error::Error;

use crate::health::repository::Repository;

// Traits

pub trait Service: Send + Sync {
    fn get_double(&self) -> usize;
}
#[derive(Provider)]
#[shaku(interface = Service)]
pub struct ServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn Repository>
}

impl Service for ServiceImpl {
    fn get_double(&self) -> usize {
        self.repo.get() * 2
    }
}