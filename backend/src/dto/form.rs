use crate::model::form;
use chrono::{NaiveDate, NaiveDateTime};
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
    pub form: form::Model,
}
