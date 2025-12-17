use eyre::Result;
use fred::{clients, prelude::*};
use std::{env, time::Duration};

pub async fn create_cache() -> Result<clients::Pool> {
    let pool_size = env::var("REDIS_POOL_SIZE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(8);
    let redis_url = env::var("REDIS_URL").expect("Failed to create redis config from url");
    let config = Config::from_url(&redis_url).expect("Failed to create redis config from url");
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
