use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::section;

#[derive(Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SectionRequest {
    pub event_id: Uuid,
    pub title: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize, PartialEq, ToSchema)]
pub struct UpdateSectionRequest {
    pub id: Uuid,
    pub title: Option<String>,
    pub price: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, ToSchema, Debug)]
pub struct SectionResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub title: String,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // pub section: section::Model,
}

impl From<section::Model> for SectionResponse {
    fn from(value: section::Model) -> Self {
        Self {
            id: value.id,
            event_id: value.event_id,
            title: value.title,
            price: value.price,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
