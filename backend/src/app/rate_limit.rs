use std::time::Duration;

use tower_governor::{
    GovernorLayer,
    governor::{GovernorConfig, GovernorConfigBuilder},
    key_extractor::{PeerIpKeyExtractor, SmartIpKeyExtractor},
};

pub fn create_rate_limit_config() {
    // rate limiter

    governor_conf
}
