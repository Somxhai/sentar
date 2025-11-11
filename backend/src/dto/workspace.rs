use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::workspace;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct WorkspaceRequest {
    pub name: String,
    pub owner_id: String,
}

#[derive(Deserialize, ToSchema, Serialize)]
pub struct RenameRequest {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
pub struct DeleteResponse {
    pub rows_affected: u64,
}

#[derive(Serialize, ToSchema, Deserialize, Debug, PartialEq)]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<workspace::Model> for WorkspaceResponse {
    fn from(workspace: workspace::Model) -> Self {
        Self {
            owner_id: workspace.owner_id,
            id: workspace.id,
            name: workspace.name,
            created_at: workspace.created_at,
            updated_at: workspace.updated_at,
        }
    }
}
