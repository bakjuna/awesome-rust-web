use std::{
    env, error::Error, net::{IpAddr, Ipv4Addr}
};

use dotenvy::from_path;
use shaku::{Component, HasComponent, Interface, Module, Provider};

// use crate::ConnectionPool;

#[derive(Debug, Clone)]
pub struct EnvProvider {
    pub postgres: Postgres,
    pub server: Server,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub address: IpAddr,
    pub port: u16,
}
#[derive(Debug, Clone)]
pub struct Postgres {
    pub user: String,
    pub password: String,
    pub database: String,
    pub host: String,
    pub port: String,
    pub schema: String,
}
pub trait Env: Interface {
    fn get(&self) -> EnvProvider;
}
#[derive(Component)]
#[shaku(interface = Env)]
pub struct EnvComponent {
    #[shaku(default=create_env())]
    env: EnvProvider,
}
impl Env for EnvComponent {
    fn get(&self) -> EnvProvider {
        self.env.clone()
    }
}

pub fn create_env() -> EnvProvider {
    from_path("backend/.env.local").ok();
    EnvProvider {
        postgres: Postgres {
            user: env::var("POSTGRES_USER").unwrap_or("yachos".to_string()),
            password: env::var("POSTGRES_PASSWORD").unwrap_or("password".to_string()),
            host: env::var("POSTGRES_HOST").unwrap_or("database".to_string()),
            database: env::var("POSTGRES_DATABASE").unwrap_or("public".to_string()),
            port: env::var("POSTGRES_PORT").unwrap_or("5432".to_string()),
            schema: env::var("POSTGRES_SCHEMA").unwrap_or("public".to_string()),
        },
        server: Server {
            address: env::var("SERVER_ADDRESS")
                .unwrap_or("0.0.0.0".to_string())
                .clone()
                .parse::<IpAddr>()
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            port: env::var("SERVER_PORT")
                .unwrap_or("8080".to_string())
                .parse::<u16>()
                .unwrap_or(8080),
        },
    }
}
impl<M: Module + HasComponent<dyn Env>> Provider<M> for EnvProvider {
    type Interface = EnvProvider;

    fn provide(module: &M) -> Result<Box<EnvProvider>, Box<dyn Error>> {
        let env_impl = module.resolve_ref().get();
        Ok(Box::new(env_impl))
    }
}