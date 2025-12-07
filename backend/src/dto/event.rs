use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, ToSchema)]
pub struct EventRequest {
    pub title: String,
    pub workspace_id: Uuid,
    pub description: Option<String>,
    pub starts_at: Option<NaiveDateTime>,
    pub ends_at: Option<NaiveDateTime>,
    pub settings: Option<Value>,
}

#[derive(Serialize, Deserialize, PartialEq, ToSchema)]
pub struct UpdateEventRequest {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub starts_at: Option<NaiveDateTime>,
    pub ends_at: Option<NaiveDateTime>,
    pub settings: Option<Value>,
}

#[derive(Serialize, Deserialize, PartialEq, ToSchema)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub workspace_id: Uuid,
    pub description: Option<String>,
    pub starts_at: Option<NaiveDateTime>,
    pub ends_at: Option<NaiveDateTime>,
    pub settings: Option<Value>,
}

impl From<crate::model::event::Model> for EventResponse {
    fn from(value: crate::model::event::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            workspace_id: value.workspace_id,
            description: value.description,
            starts_at: value.starts_at,
            ends_at: value.ends_at,
            settings: value.settings,
        }
    }
}
