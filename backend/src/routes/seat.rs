use axum::{Router, routing::post};

pub fn seat_routes() -> Router {
    Router::new().route("/place", post(place_seat))
}

async fn place_seat() {}
