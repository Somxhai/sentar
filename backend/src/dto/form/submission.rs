use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, TryIntoModel};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{dto::connection_info::ConnectionInfo, model::form_submission};

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct FormSubmit {
    pub submission_id: Option<Uuid>,
    // pub form_version_id: Uuid,
    #[schema(value_type = Object)]
    pub answers: Value,
    #[serde(default)]
    pub is_draft: bool,
}

impl FormSubmit {
    pub async fn submit(
        self,
        db: &DatabaseConnection,
        user_id: String,
        form_id: Uuid,
        connect_info: ConnectionInfo,
    ) -> eyre::Result<form_submission::Model> {
        let status = if self.is_draft { "draft" } else { "submitted" };
        let mut submission = form_submission::ActiveModel {
            answer: Set(self.answers),
            respondent_id: Set(user_id),
            status: Set(status.into()),
            form_id: Set(form_id),
            user_agent: Set(Some(connect_info.user_agent.to_string())),
            ip_address: Set(Some(connect_info.ip.to_string())),
            ..Default::default()
        };

        // Update if id is specified.
        if let Some(submission_id) = self.submission_id {
            submission.id = Set(submission_id);
        }

        let submission = submission.save(db).await?;

        Ok(submission.try_into_model()?)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct FormSubmitResponse {
    pub id: Uuid,
    pub respondent_id: String,
    pub form_id: Uuid,
    pub answer: Value,
    pub submitted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

impl IntoResponse for FormSubmitResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

impl From<form_submission::Model> for FormSubmitResponse {
    fn from(value: form_submission::Model) -> Self {
        Self {
            id: value.id,
            respondent_id: value.respondent_id,
            form_id: value.form_id,
            answer: value.answer,
            submitted_at: value.submitted_at,
            updated_at: value.updated_at,
            status: value.status,
            user_agent: value.user_agent,
            ip_address: value.ip_address,
        }
    }
}
