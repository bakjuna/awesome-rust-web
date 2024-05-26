pub use self::
    errors::BootError
;
use crate::app_state::{AppState, AppModule};
use crate::health::route::router_health;
use auth::route::router_auth;
use axum::{middleware, Router};
use cron::component::CronJobInterface;
use env::{create_env, EnvProvider};
use shaku::HasComponent;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use crate::errors::BootResult;

mod app_state;
mod database;
mod errors;
mod health;
mod logs;
mod middlewares;
mod env;
mod cron;
mod auth;

#[tokio::main]
async fn main() -> BootResult {
    let raw_module = AppModule::builder().build();
    let cr_start: Arc<dyn CronJobInterface> = raw_module.resolve();
    cr_start.initialize();
    let module = Arc::new(raw_module);
    let state = AppState { module };
    // migrate(&state.module).await;
    let app = create_routes(state);
    let addr = create_addr();
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await;
    match server {
        Ok(app) => Ok(app),
        Err(_err) => Err(BootError::Api),
    }
}


fn create_routes(app_state: AppState) -> Router {
    Router::new()
        .nest("/healthz", router_health())
        .nest("/auth", router_auth())
        .layer(middleware::map_response(
            middlewares::middleware::main_response_mapper,
        ))
        .with_state(app_state)
}

fn create_addr() -> SocketAddr {
    let env: EnvProvider = create_env();
    let ip_addr: IpAddr = env.server.address;
    let port: u16 = env.server.port;
    SocketAddr::new(ip_addr, port)
}