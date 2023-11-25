use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
use std::cell::RefCell;
use std::error::Error;
use crate::DBConnection;

// Traits

pub trait Repository: Send + Sync {
    fn get(&self) -> usize;
}
#[derive(Provider)]
#[shaku(interface = Repository)]
pub struct RepositoryImpl {
    #[shaku(provide)]
    db: Box<DBConnection>
}

impl Repository for RepositoryImpl {
    fn get(&self) -> usize {
        (*self.db).0
    }
}
