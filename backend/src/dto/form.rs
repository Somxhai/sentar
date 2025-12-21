use crate::model::form;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FormRequest {
    pub event_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateFormRequest {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct FormResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub schema: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<form::Model> for FormResponse {
    fn from(m: form::Model) -> Self {
        Self {
            id: m.id,
            event_id: m.event_id,
            schema: m.schema,
            settings: m.settings,
            title: m.title,
            description: m.description,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}
