pub use self::{
    errors::{BootError, Result},
    logs::log_request,
};
use crate::app_state::{AppState, AppModule};
use crate::health::route::router_health;
use auth::route::router_auth;
use axum::{extract::State, middleware, Router};
use shaku::HasComponent;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use crate::env::Env;
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

    let module = Arc::new(AppModule::builder().build());
    let state = AppState { module };
    // migrate(&state.module).await;
    handle_cronjob(&state).await;
    let env: &dyn Env = state.module.resolve_ref();
    let ip_addr: IpAddr = env.get().server.address;
    let port: u16 = env.get().server.port;
    let app = Router::new()
        .nest("/healthz", router_health(state.clone()))
        .nest("/auth", router_auth(state.clone()))
        .layer(middleware::map_response(
            middlewares::middleware::main_response_mapper,
        ));
    let addr: SocketAddr = SocketAddr::new(ip_addr, port);
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await;
    match server {
        Ok(app) => Ok(app),
        Err(_err) => Err(BootError::Api),
    }
}

async fn handle_cronjob(app_state: &AppState) {
    println!("Creating Cronjobs...");
    let cron_jobs = cron::creator::create_cron_jobs(app_state).await.unwrap();
    println!("Creating Cronjobs Completed");
    cron_jobs.start().await.unwrap();
}
