use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

use shaku::{Component, Interface, Module, HasComponent};

use crate::ConnectionPool;

pub trait Env: Interface {
    fn default(&self) -> EnvImpl;
}
pub struct EnvImpl {
    pub postgres: Postgres,
    pub server: Server,
}
pub struct Server {
    pub address: IpAddr,
    pub port: u16,
}

pub struct Postgres {
    pub user: String,
    pub password: String,
    pub database: String,
    pub host: String,
    pub port: String,
}
impl<M: Module> Component<M> for EnvImpl {
    type Interface = dyn Env;
    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        params: Self::Parameters,
    ) -> Box<Self::Interface> {
        Box::new(Self {
            postgres: Postgres {
                user: env::var("POSTGRES_USER").unwrap_or("yacho".to_string()),
                password: env::var("POSTGRES_PASSWORD").unwrap_or("password".to_string()),
                host: env::var("POSTGRES_HOST").unwrap_or("database".to_string()),
                database: env::var("POSTGRES_DATABASE").unwrap_or("public".to_string()),
                port: env::var("POSTGRES_PORT").unwrap_or("5432".to_string()),
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
        })
    }
}
impl Env for EnvImpl {
    fn default(&self) -> Self {
        Self {
            postgres: Postgres {
                user: env::var("POSTGRES_USER").unwrap_or("yacho".to_string()),
                password: env::var("POSTGRES_PASSWORD").unwrap_or("password".to_string()),
                host: env::var("POSTGRES_HOST").unwrap_or("database".to_string()),
                database: env::var("POSTGRES_DATABASE").unwrap_or("public".to_string()),
                port: env::var("POSTGRES_PORT").unwrap_or("5432".to_string()),
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
}
