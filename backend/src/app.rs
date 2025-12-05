use std::{env, process, sync::Arc, time::Duration};

use crate::routes::{
    event::event_routes,
    form::form_routes,
    health_check,
    section::section_routes,
    workspace::{workspace_routes, workspaces::workspaces_routes},
};
use axum::{Router, routing::get};
use axum_prometheus::PrometheusMetricLayer;
use eyre::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing_loki::url::Url;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

impl AppState {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        AppState { db }
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

pub fn create_router(db: DatabaseConnection) -> Result<Router> {
    let app_state = AppState::new(Arc::new(db));
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    // Build router and OpenAPI spec
    let (router, api): (Router, utoipa::openapi::OpenApi) = OpenApiRouter::<AppState>::new()
        .merge(workspace_routes())
        .merge(workspaces_routes())
        .merge(event_routes())
        .merge(section_routes())
        .merge(form_routes())
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/health", get(health_check))
        .layer(prometheus_layer)
        .with_state(app_state.clone())
        .split_for_parts();

    // Merge Swagger UI route
    let app = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    Ok(app)
}
