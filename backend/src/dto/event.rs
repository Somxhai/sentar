use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    dto::form::FormResponse,
    model::{event, event_object},
};

#[derive(Serialize, Deserialize, PartialEq, ToSchema, Default)]
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

#[derive(Serialize, Deserialize, PartialEq, ToSchema, Debug)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventPosition {
    pub id: Uuid,
    pub event_object_id: Uuid,
    pub position_x: f64,
    pub position_y: f64,
    pub rotation: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventEntry {
    pub forms: Vec<FormResponse>,
    pub objects: Vec<EventObject>,
    pub event: EventResponse,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventObject {
    pub id: Uuid,
    pub object_type: String,
    pub event_id: Uuid,
    pub section_id: Option<Uuid>,
    pub label: Option<String>,
    pub is_enable: bool,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<event::ModelEx> for EventEntry {
    fn from(value: event::ModelEx) -> Self {
        let event_part: EventResponse = value.clone().into();
        Self {
            forms: value.forms.into_iter().map(Into::into).collect(),
            objects: value.event_objects.into_iter().map(Into::into).collect(),
            event: event_part,
        }
    }
}

impl From<event::ModelEx> for EventResponse {
    fn from(value: event::ModelEx) -> Self {
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

impl From<event_object::ModelEx> for EventObject {
    fn from(value: event_object::ModelEx) -> Self {
        Self {
            id: value.id,
            object_type: value.object_type,
            event_id: value.event_id,
            section_id: value.section_id,
            label: value.label,
            is_enable: value.is_enable,
            status: value.status,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
