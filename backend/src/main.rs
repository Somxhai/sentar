use std::net::SocketAddr;

use app::cache::create_cache;
use app::create_database;
use app::create_router;
use eyre::Result;
use observe::create_logging_provider;
use observe::create_oltp_provider;
use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::error;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::env_vars::AppConfig;

pub mod app;
pub mod dto;
pub mod env_vars;
pub mod error;
pub mod middleware;
pub mod model;
mod observe;
pub mod prometheus;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    AppConfig::init();
    info!("Initializing app");
    global::set_text_map_propagator(TraceContextPropagator::new());
    let (toki_layer, task) = create_logging_provider()?;
    tokio::spawn(task);

    let otel_layer = create_oltp_provider()?;

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(toki_layer)
        .with(tracing_subscriber::fmt::Layer::new())
        // .with(EnvFilter::new("debug"))
        .init();

    let db = create_database().await?;
    let cache = create_cache().await?;

    let app = create_router(
        db, cache, false, // jwks
    )?;
    let port: u16 = AppConfig::global().port.unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Server runs on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap_or_else(|err| error!("Cannot start the server: {}", err));
    Ok(())
}
