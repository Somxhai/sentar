use std::process;

use eyre::Result;
use opentelemetry::global::{self, BoxedTracer};
use opentelemetry_otlp::WithExportConfig;
use tracing_loki::{BackgroundTask, Layer, url::Url};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::Registry;

pub fn create_oltp_provider() -> Result<OpenTelemetryLayer<Registry, BoxedTracer>> {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()?;

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .build();

    global::set_tracer_provider(provider);
    let tracer = global::tracer("sentar_trace");
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    Ok(otel_layer)
}

pub fn create_logging_provider() -> Result<(Layer, BackgroundTask)> {
    let (layer, task) = tracing_loki::builder()
        .label("service", "sentar-backend")?
        .label("environment", "development")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    Ok((layer, task))
}
