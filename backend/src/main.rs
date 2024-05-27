use backend::{
    create_server,
    errors::{BootError, BootResult},
};

#[tokio::main]
async fn main() -> BootResult {
    match create_server().await {
        Ok(app) => Ok(app),
        Err(_err) => Err(BootError::Api),
    }
}
