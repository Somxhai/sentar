use std::sync::{Once, OnceLock};

use axum_client_ip::ClientIpSource;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub port: Option<u16>,
    pub redis_pool_size: Option<usize>,
    pub ip_source: ClientIpSource,
    pub database_url: String,
    pub redis_url: String,
    pub otel_service_name: String,
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();
static INIT: Once = Once::new();
impl AppConfig {
    pub fn global() -> &'static AppConfig {
        CONFIG.get().expect("AppConfig is not initialized")
    }

    pub fn init() {
        INIT.call_once(|| {
            let config =
                envy::from_env::<AppConfig>().expect("Failed to load environment variables");

            CONFIG
                .set(config)
                .expect("Config has already been initialized");
        });
    }
}
