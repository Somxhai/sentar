use axum::{extract::State, http::StatusCode};
use tracing::{Instrument, info_span};

use crate::{app::AppState, error::AppError};

pub mod event;
pub mod form;
pub mod form_submission;
pub mod section;
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

// #[utoipa::path(
//     get,
//     path = "/token/{token}",
//     params(
//         ("token" = String, Path, description = "JWT token")
//     ),
//     responses(
//         (status = 200, description = "Token decoded"),
//         (status = 401, description = "Unauthorized")
//     )
// )]
// #[tracing::instrument(name = "test_jwt", skip(app_state))]
// pub async fn test_jwt(
//     header: HeaderMap,
//     Path(token): Path<String>,
//     State(app_state): State<AppState>,
// ) -> Result<Json<serde_json::Value>, AppError> {
//     debug!("JWKS: {:?}", app_state.jwks);
//     let jwks = app_state.jwks.read().await;
//     let token_data = create_jwt_verifier(&token, &*jwks).map_err(|_| AppError::Unauthorized)?;
//
//     let betterauth = header
//         .get("cookie")
//         .and_then(|v| v.to_str().ok())
//         .and_then(|cookie_str| {
//             cookie_str
//                 .split(';')
//                 .map(str::trim)
//                 .find(|c| c.starts_with("better-auth"))
//                 .map(|s| s.to_string())
//         });
//     Ok(Json(json!({
//         "header": token_data.header,
//         "claims": token_data.claims,
//         "better-auth": betterauth,
//     })))
// }
