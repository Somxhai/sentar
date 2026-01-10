use std::net::IpAddr;

use axum_extra::headers::UserAgent;

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
    pub ip: IpAddr,
    pub user_agent: UserAgent,
}
