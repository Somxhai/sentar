use eyre::Result;
use fred::{clients, mocks::SimpleMap, prelude::*};
use std::{sync::Arc, time::Duration};

use crate::env_vars::AppConfig;

pub async fn create_cache() -> Result<clients::Pool> {
    let pool_size = AppConfig::global().redis_pool_size.unwrap_or(8);
    let redis_url = &AppConfig::global().redis_url;
    let config = Config::from_url(redis_url).expect("Failed to create redis config from url");
    let pool = Builder::from_config(config)
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(10);
        })
        // use exponential backoff, starting at 100 ms and doubling on each failed attempt up to 30 sec
        .set_policy(ReconnectPolicy::new_exponential(0, 100, 30_000, 2))
        .build_pool(pool_size)
        .expect("Failed to create redis pool");

    pool.init().await.expect("Failed to connect to redis");

    Ok(pool)
}

pub async fn create_test_cache() -> Result<clients::Pool> {
    let config = Config {
        mocks: Some(Arc::new(SimpleMap::new())),
        ..Default::default()
    };
    let pool = Builder::from_config(config).build_pool(5).unwrap();
    pool.init().await.expect("Failed to connect");
    Ok(pool)
}
