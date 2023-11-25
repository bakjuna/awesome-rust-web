use std::{
    net::{IpAddr, SocketAddr, Ipv4Addr},
    sync::Arc,
};

// use crate::health::service::HealthService;

pub use self::app_state::*;
pub use self::database::*;
pub use self::{
    errors::{BootError, Error, Result},
    logs::log_request,
};
use axum::{middleware, routing::get, Router};
use errors::BootResult;
use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
mod app_state;
mod database;
mod errors;
mod health;
mod logs;
mod env;
mod middlewares;

async fn root() -> &'static str {
    "Hello, World!"
}
pub fn test_router(state: AppState) -> Router {
    Router::new().route("/", get(root))
    // .with_state(state)
}

#[tokio::main]
async fn main() -> BootResult {
    println!("Starting Server...");
    let module = Arc::new(ExampleModule::builder().build());
    // let module: Arc<AppModule> = Arc::new(AppModule::builder().build());
    let state = AppState { module };

    let ip_addr: IpAddr = "127.0.0.1"
        .clone()
        .parse::<IpAddr>()
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port: u16 = "8080".parse::<u16>().unwrap_or(8080);

    let routes_all: Router = Router::new()
        .nest("/", health::route::router_health(state))
        .layer(middleware::map_response(
            middlewares::middleware::main_response_mapper,
        ));
    let addr: SocketAddr = SocketAddr::new(ip_addr, port);
    println!("->> LISTENING on {addr} \n");
    let server = axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await;
    match server {
        Ok(app) => Ok(app),
        Err(_err) => Err(BootError::Api),
    }
}
