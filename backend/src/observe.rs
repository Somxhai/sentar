use std::process;

use eyre::Result;
use opentelemetry::global;
use tracing_loki::{BackgroundTask, Layer, url::Url};

pub fn create_oltp_provider() -> Result<()> {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(otlp_exporter)
        .build();

    global::set_tracer_provider(provider);
    Ok(())
}

pub fn create_logging_provider() -> Result<(Layer, BackgroundTask)> {
    let (layer, task) = tracing_loki::builder()
        .label("service", "sentar-backend")?
        .label("environment", "development")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    Ok((layer, task))
}
