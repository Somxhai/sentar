use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request},
    http::{HeaderValue, header},
    middleware::{self, Next},
};
use axum_extra::extract::cookie::Cookie;
use axum_test::TestServer;
use backend::{
    app::{cache::create_test_cache, create_router},
    dto::cache::SessionCache,
    env_vars::AppConfig,
};
use chrono::{Duration, Utc};
use eyre::Result;
use fred::prelude::KeysInterface;
use sea_orm::MockDatabase;

pub async fn create_test_app(mock_db: MockDatabase) -> Result<TestServer> {
    dotenv::dotenv().ok();
    AppConfig::init();
    let db = mock_db.into_connection();
    let cache = create_test_cache().await?;

    let fake_token = "fake-token-xxx";
    let cache_key = format!("session:{}", fake_token);

    let session = &SessionCache {
        user_id: "my_user_id".to_string(),
        expires_at: (Utc::now() + Duration::hours(1)).naive_utc(),
    };
    let session_json = serde_json::to_string(&session).unwrap();

    let _: () = cache
        .set(&cache_key, session_json, None, None, false)
        .await?;

    let app = create_router(db, cache, true)?;

    let app_with_mock_ip = app.layer(middleware::from_fn(
        |mut req: Request, next: Next| async move {
            let addr = SocketAddr::from(([127, 0, 0, 1], 12345));
            req.extensions_mut().insert(ConnectInfo(addr));

            req.headers_mut().insert(
                header::USER_AGENT,
                HeaderValue::from_static("TestRunner/1.0"),
            );

            next.run(req).await
        },
    ));

    let mut server = TestServer::builder()
        .http_transport()
        .build(app_with_mock_ip)
        .unwrap();
    let cookie = Cookie::build(("better-auth.session_token", fake_token))
        .path("/")
        .http_only(true)
        .secure(false)
        .build();
    server.add_cookie(cookie);
    Ok(server)
}
