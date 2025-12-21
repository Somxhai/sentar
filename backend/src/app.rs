use crate::{
    middleware::auth::auth_session_guard,
    routes::{
        event::event_routes,
        form::form_routes,
        health_check,
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
use tower_http::cors::CorsLayer;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

pub mod cache;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub cache: Arc<fred::prelude::Pool>, // pub jwks: Arc<RwLock<JwkSet>>,
}

impl AppState {
    pub fn new(
        db: Arc<DatabaseConnection>,
        cache: Arc<fred::prelude::Pool>, // jwks: Arc<RwLock<JwkSet>>
    ) -> Self {
        AppState {
            db,
            cache, // jwks
        }
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
    // jwks: JwkSet
) -> Result<Router> {
    let db = Arc::new(db);
    let cache_pool = Arc::new(cache_pool);
    // let jwks = Arc::new(RwLock::new(jwks));
    let app_state = AppState::new(
        db, cache_pool, // jwks
    );
    let (prometheus_layer, metric_handle) =
        PROMETHEUS.get_or_init(PrometheusMetricLayer::pair).clone();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_credentials(true);

    let public = OpenApiRouter::<AppState>::new()
        .route("/health", get(health_check))
        .route("/metrics", get(|| async move { metric_handle.render() }));

    let protected = OpenApiRouter::<AppState>::new()
        .merge(workspace_routes())
        .merge(workspaces_routes())
        .merge(event_routes())
        .merge(section_routes())
        .merge(form_routes())
        .layer(from_fn_with_state(app_state.clone(), auth_session_guard));

    // Build router and OpenAPI spec
    let (router, api): (Router, utoipa::openapi::OpenApi) = public
        .merge(protected)
        .layer(prometheus_layer)
        .layer(OtelInResponseLayer) // Log
        .layer(OtelAxumLayer::default()) // Trace
        .with_state(app_state.clone())
        .layer(cors)
        .split_for_parts();

    // Merge Swagger UI route
    let app = router
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", &api))
        .merge(Scalar::with_url("/docs", api));

    Ok(app)
}
