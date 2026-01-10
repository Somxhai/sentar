use axum::{extract::State, http::StatusCode};
use tracing::{Instrument, info_span};

use crate::{app::AppState, error::AppError};

pub mod event;
pub mod form;
pub mod form_submission;
pub mod section;
pub mod websocket;
pub mod workspace;

#[utoipa::path(
    get,
    path = "/health",
    tag = "public",
    responses(
        (status = 200, description = "App is healthy")
    )

)]
pub async fn health_check(State(app_state): State<AppState>) -> Result<StatusCode, AppError> {
    app_state
        .db
        .ping()
        .instrument(info_span!("db.ping"))
        .await?;
    Ok(StatusCode::OK)
}
