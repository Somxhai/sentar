use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use tracing::debug;

use crate::{app::AppState, error::AppError, utils::auth::validate_token};

pub async fn auth_session_guard(
    jar: CookieJar,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let session_token = jar
        .get("better-auth.session_token")
        .map(|token| token.value())
        .ok_or(AppError::Unauthorized)?;

    debug!(session_token);

    let db = &state.db;
    let cache = &state.cache;

    let session_cache = validate_token(session_token, db, cache).await?;

    req.extensions_mut().insert(session_cache);
    Ok(next.run(req).await)
}
