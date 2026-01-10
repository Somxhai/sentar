use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum ServerEvent {
    SeatReserved {
        seat_ids: Vec<Uuid>,
        by: String,
    },
    SeatReleased {
        seat_id: Uuid,
    },
    SeatMoved {
        seat_id: Uuid,
        x: f64,
        y: f64,
        z: f64,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum BroadcastMessage {
    Ack {
        request_id: String,
    },
    Error {
        request_id: String,
        code: String,
        message: String,
    },
    Event {
        event: ServerEvent,
    },
}

#[allow(dead_code)]
pub enum ActionResult {
    Ack {
        ack: BroadcastMessage,
    },
    AckAndBroadcast {
        ack: BroadcastMessage,
        event: ServerEvent,
    },
    Error {
        request_id: String,
        code: String,
        message: String,
    },
}
