use axum::{extract::Request, middleware::Next, response::Response};
use axum_client_ip::ClientIp;
use axum_extra::{TypedHeader, headers::UserAgent};
use tracing::info;

use crate::dto::connection_info::ConnectionInfo;

pub async fn retrieve_connection_info_middleware(
    ClientIp(ip): ClientIp,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    mut req: Request,
    next: Next,
) -> Response {
    let connection_info = ConnectionInfo { ip, user_agent };
    info!("connection_info: {:?}", connection_info);
    req.extensions_mut().insert(connection_info);

    next.run(req).await
}
