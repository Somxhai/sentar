use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error")]
    Internal,

    #[error("Database Conflict: {0}")]
    Conflict(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

// Implement IntoResponse for your error type
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error",
                    Some(e.to_string()),
                )
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "Not found", Some(msg)),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized", None),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "Bad request", Some(msg)),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "Validation error", Some(msg)),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "Conflict", Some(msg)),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                None,
            ),
        };

        let body = Json(ErrorResponse {
            error: message.to_string(),
            details,
        });

        (status, body).into_response()
    }
}
