use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum WSMessage {
    Reserve {
        request_id: String,
        seat_ids: Vec<Uuid>,
    },
    Release {
        request_id: String,
        seat_id: Uuid,
    },
    MoveObjectInLayout {
        request_id: String,
        seat_id: Uuid,
        x: f64,
        y: f64,
        z: f64,
    },
}
