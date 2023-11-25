use std::{sync::Arc, net::{IpAddr, SocketAddr}};

use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
pub use self::{errors::{BootError, Error, Result}, logs::log_request};
pub use self::app_state::*;
use errors::BootResult;
use axum::{middleware, Router, routing::get};
mod health;
mod errors;
mod app_state;
mod logs;
mod middlewares;

#[tokio::main]
async fn main() -> BootResult {
    println!("Starting Server...");
    let module: Arc<AppModule> = Arc::new(AppModule::builder().build());
    let state = AppState { module };

    // let ip_addr: IpAddr = app_state.get_env().server.address;
    // let port: u16 = app_state.get_env().server.port;

    let routes_all: Router = Router::new()
        .nest("/", health::route::router_health())
        .layer(middleware::map_response(
            middlewares::middleware::main_response_mapper,
        ));

    let server = axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
        .serve(routes_all.into_make_service())
        .await;
    match server {
        Ok(app) => Ok(app),
        Err(_err) => Err(BootError::Api),
    }
}