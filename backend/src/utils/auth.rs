use chrono::NaiveDateTime;
use fred::{
    prelude::{KeysInterface, Pool},
    types::Expiration,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::{error, warn};

use crate::{dto::cache::SessionCache, error::AppError, model::session};

pub async fn validate_token(
    token: &str,
    db: &DatabaseConnection,
    cache: &Pool,
) -> Result<SessionCache, AppError> {
    let redis_key = format!("session:{}", token);

    let cached_session = match cache.get::<Option<String>, _>(&redis_key).await {
        Ok(Some(json)) => serde_json::from_str::<SessionCache>(&json).ok(),
        Ok(None) => None,
        Err(e) => {
            warn!("Redis connection error: {}", e);
            None
        }
    };

    if let Some(session_data) = cached_session
        && session_data.expires_at > chrono::Utc::now().naive_utc()
    {
        return Ok(session_data);
    }

    let session = session::Entity::find()
        .filter(session::Column::Token.eq(token))
        .filter(session::Column::ExpiresAt.gt(chrono::Utc::now()))
        .one(db)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let session_cache = SessionCache {
        user_id: session.user_id,
        expires_at: session.expires_at,
    };
    let session_cache_clone = session_cache.clone();

    let cache = cache.clone();
    let _save_to_cache = tokio::spawn(async move {
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

    Ok(session_cache)
}

fn calculate_ttl(expires_at: NaiveDateTime) -> Expiration {
    let ttl = (expires_at - chrono::Utc::now().naive_utc())
        .num_seconds()
        .max(0);
    Expiration::EX(ttl.min(5 * 60)) // Cap at 5 minutes
}
