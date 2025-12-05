use axum::{extract::State, http::StatusCode};
use tracing::{Instrument, info_span};

use crate::{app::AppState, error::AppError};

pub mod event;
pub mod form;
pub mod section;
pub mod workspace;

pub async fn health_check(State(app_state): State<AppState>) -> Result<StatusCode, AppError> {
    let span = info_span!("health_check");
    async move {
        app_state
            .db
            .ping()
            .instrument(info_span!("health_check.db.ping"))
            .await?;
        Ok(StatusCode::OK)
    }
    .instrument(span)
    .await
}
