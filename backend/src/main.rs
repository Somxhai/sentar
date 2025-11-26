use std::net::SocketAddr;
use std::process;

use backend::app::create_database;
use eyre::Result;
use tracing::error;
use tracing::info;
use tracing_loki::url::Url;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::app::create_router;

pub mod app;
pub mod dto;
pub mod error;
pub mod model;
pub mod prometheus;
pub mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let (layer, task) = tracing_loki::builder()
        .label("host", "mine")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    tracing_subscriber::registry()
        .with(layer)
        .with(tracing_subscriber::fmt::Layer::new())
        .init();
    tokio::spawn(task);
    dotenv::dotenv().ok();

    let db = create_database().await?;

    let app = create_router(db)?;
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Server runs on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| error!("Cannot start the server: {}", err));
    Ok(())
}
