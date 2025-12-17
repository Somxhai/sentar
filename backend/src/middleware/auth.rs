use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use chrono::NaiveDateTime;
use fred::{prelude::KeysInterface, types::Expiration};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, warn};

use crate::{app::AppState, error::AppError, model::session};

#[derive(Serialize, Debug, Clone, Deserialize)]
struct SessionCache {
    user_id: String,
    expires_at: NaiveDateTime,
}

pub async fn auth_session_guard(
    jar: CookieJar,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    use crate::model::session::Entity;
    let session_token = jar
        .get("better-auth.session_token")
        .map(|token| token.value())
        .ok_or(AppError::Unauthorized)?;

    debug!(session_token);

    let db = &*state.db;
    let cache = state.cache.clone();

    let redis_key = format!("session:{}", session_token);

    let cached_session = match cache.get::<String, _>(&redis_key).await {
        Ok(json) => serde_json::from_str::<SessionCache>(&json).ok(),
        Err(e) => {
            warn!("Cache read error for session {}: {}", session_token, e);
            None
        }
    };

    if let Some(session_data) = cached_session {
        if session_data.expires_at > chrono::Utc::now().naive_utc() {
            req.extensions_mut().insert(session_data);
            return Ok(next.run(req).await);
        }
    }

    let session = Entity::find()
        .filter(session::Column::Token.eq(session_token))
        .filter(session::Column::ExpiresAt.gt(chrono::Utc::now()))
        .one(db)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let session_cache = SessionCache {
        user_id: session.user_id,
        expires_at: session.expires_at,
    };
    let session_cache_clone = session_cache.clone();

    tokio::spawn(async move {
        let ttl = calculate_ttl(session_cache_clone.expires_at);
        match serde_json::to_string(&session_cache_clone) {
            Ok(json_str) => {
                let result: Result<(), _> = cache
                    .set(&redis_key, json_str, Some(ttl), None, false)
                    .await;
                if let Err(e) = result {
                    error!("Failed to cache session: {}", e);
                }
            }
            Err(e) => {
                error!("Session serialization failed: {}", e);
            }
        };
    });

    req.extensions_mut().insert(session_cache);
    Ok(next.run(req).await)
}

fn calculate_ttl(expires_at: NaiveDateTime) -> Expiration {
    let ttl = (expires_at - chrono::Utc::now().naive_utc())
        .num_seconds()
        .max(0);
    Expiration::EX(ttl.min(5 * 60)) // Cap at 5 minutes
}
