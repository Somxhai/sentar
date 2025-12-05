use std::net::SocketAddr;
use std::process;

use crate::app::create_router;
use backend::app::create_database;
use eyre::Result;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_stdout::LogExporter;
use tracing::error;
use tracing::info;
use tracing_loki::url::Url;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod app;
pub mod dto;
pub mod error;
pub mod model;
pub mod prometheus;
pub mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let (layer, task) = tracing_loki::builder()
        .label("service", "sentar-backend")?
        .label("environment", "development")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    let exporter = LogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
    let otel_layer = OpenTelemetryTracingBridge::new(&provider);
    tokio::spawn(task);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(layer)
        .with(tracing_subscriber::fmt::Layer::new())
        .init();

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
