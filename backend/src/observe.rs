use std::process;

use eyre::Result;
use opentelemetry::{
    KeyValue,
    global::{self, BoxedTracer},
};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use tracing_loki::{BackgroundTask, Layer, url::Url};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::Registry;

pub fn create_oltp_provider() -> Result<OpenTelemetryLayer<Registry, BoxedTracer>> {
    let resource = Resource::builder()
        .with_service_name("sentar-backend")
        .with_attribute(KeyValue::new("env", "development"))
        .build();

    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_export_config(ExportConfig {
            endpoint: Some("http://localhost:4317".into()),
            protocol: opentelemetry_otlp::Protocol::Grpc,
            ..Default::default()
        })
        .build()?;

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .with_resource(resource)
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .build();

    let tracer = global::tracer("sentar_trace");
    global::set_tracer_provider(provider);
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
