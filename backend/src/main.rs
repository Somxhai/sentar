use eyre::Result;
use std::net::SocketAddr;
use tracing::error;

use crate::app::create_router;

pub mod app;
pub mod error;
pub mod model;
pub mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv().ok();

    let app = create_router().await?;
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| error!("Cannot start the server: {}", err));
    Ok(())
}
