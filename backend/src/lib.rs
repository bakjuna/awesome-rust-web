pub use crate::app_state::*;
use app_state::{AppModule, AppState};
use auth::route::router_auth;
use axum::{middleware, Router};
use cron::component::CronJobInterface;
use env::{create_env, EnvProvider};
use health::route::router_health;
use shaku::HasComponent;
use std::{
    io::Error,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

pub mod app_state;
pub mod auth;
pub mod cron;
pub mod database;
pub mod env;
pub mod errors;
pub mod health;
pub mod logs;
pub mod middlewares;

pub fn create_routes(app_state: AppState) -> Router {
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

fn run_cronjob(module: &AppModule) {
    let cr_start: Arc<dyn CronJobInterface> = module.resolve();
    cr_start.initialize();
}
pub fn construct_router() -> Router {
    let raw_module = Arc::new(AppModule::builder().build());
    run_cronjob(&raw_module);
    create_from_raw_module(raw_module)
}

pub fn create_from_raw_module(raw_module: Arc<AppModule>) -> Router {
    let state = AppState { module: raw_module };
    create_routes(state)
}

pub async fn create_server() -> Result<(), Error> {
    let addr = create_addr();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, construct_router()).await
}
