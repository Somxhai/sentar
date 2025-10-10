use axum::{Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tracing::error;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // Build router and OpenAPI spec
    let (router, api): (Router, utoipa::openapi::OpenApi) =
        OpenApiRouter::new().routes(routes!(ping)).split_for_parts();

    // Merge Swagger UI route
    let app = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| error!("Cannot start the server: {}", err));
}

#[derive(Serialize, utoipa::ToSchema)]
struct PingResponse {
    message: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Ping successful", body = PingResponse)
    )
)]
async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        message: "Ping!".to_string(),
    })
}
