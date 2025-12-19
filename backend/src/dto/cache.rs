use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct SessionCache {
    pub user_id: String,
    pub expires_at: NaiveDateTime,
}
