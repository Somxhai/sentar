use crate::{
    middleware::auth::auth_session_guard,
    routes::{
        event::{event_routes, public_event_routes},
        form::{form_routes, public_form_routes},
        form_submission::form_submission_routes,
        section::section_routes,
        workspace::{workspace_routes, workspaces::workspaces_routes},
    },
};
use axum::{Router, http::HeaderValue, middleware::from_fn_with_state, routing::get};
use axum_prometheus::{PrometheusMetricLayer, metrics_exporter_prometheus::PrometheusHandle};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use eyre::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{
    env,
    sync::{Arc, OnceLock},
    time::Duration,
};
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};
use tower_http::cors::CorsLayer;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};

pub mod cache;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub cache: Arc<fred::prelude::Pool>,
}

impl AppState {
    pub fn new(db: Arc<DatabaseConnection>, cache: Arc<fred::prelude::Pool>) -> Self {
        AppState { db, cache }
    }
}

pub async fn create_database() -> Result<DatabaseConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_mins(8))
        .max_lifetime(Duration::from_mins(30))
        .sqlx_logging(true)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;
    Ok(db)
}

static PROMETHEUS: OnceLock<(PrometheusMetricLayer, PrometheusHandle)> = OnceLock::new();

pub fn create_router(
    db: DatabaseConnection,
    cache_pool: fred::prelude::Pool,
    is_test: bool,
) -> Result<Router> {
    let db = Arc::new(db);
    let cache_pool = Arc::new(cache_pool);
    let app_state = AppState::new(db, cache_pool);

    let governor_conf = GovernorConfigBuilder::default()
        .use_headers()
        .per_second(2)
        .key_extractor(SmartIpKeyExtractor)
        .burst_size(5)
        .finish()
        .unwrap();

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    // a separate background task to clean up
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    let (prometheus_layer, metric_handle) =
        PROMETHEUS.get_or_init(PrometheusMetricLayer::pair).clone();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_credentials(true);

    let public = OpenApiRouter::<AppState>::new()
        .routes(routes!(crate::routes::health_check))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .merge(public_event_routes())
        .merge(public_form_routes());

    let protected = OpenApiRouter::<AppState>::new()
        .merge(workspace_routes())
        .merge(workspaces_routes())
        .merge(event_routes())
        .merge(section_routes())
        .merge(form_routes())
        .merge(form_submission_routes())
        .layer(from_fn_with_state(app_state.clone(), auth_session_guard));

    // Build router and OpenAPI spec
    let mut router = public
        .merge(protected)
        .layer(prometheus_layer)
        .layer(OtelInResponseLayer) // Log
        .layer(OtelAxumLayer::default()) // Trace
        .with_state(app_state.clone())
        .layer(cors);

    if !is_test {
        router = router.layer(GovernorLayer::new(Arc::new(governor_conf)))
    }

    let (router, api): (Router, utoipa::openapi::OpenApi) = router.split_for_parts();

    // Merge Swagger UI route
    let app = router
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", &api))
        .merge(Scalar::with_url("/docs", api));

    Ok(app)
}
