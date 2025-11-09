use axum::extract::Path;
use axum::{Json, extract::State};
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::app::AppState;
use crate::error::AppError;
use crate::model::workspace;

pub fn workspace_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(
        create_workspace,
        get_workspaces,
        delete_workspace,
        rename_workspace,
        get_workspace
    ))
}

#[derive(Deserialize, ToSchema)]
struct WorkspaceRequest {
    name: String,
    owner_id: String,
}

#[derive(Deserialize, ToSchema)]
struct RenameRequest {
    id: Uuid,
    name: String,
}

#[derive(Serialize, ToSchema)]
struct DeleteResponse {
    rows_affected: u64,
}

#[derive(Serialize, ToSchema)]
struct WorkspaceResponse {
    id: Uuid,
    name: String,
    owner_id: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl WorkspaceResponse {
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

#[utoipa::path(
    post,
    path = "/workspace",
    tag = "workspace",
    request_body = WorkspaceRequest,
    responses((status = 200, body = WorkspaceResponse))
)]
async fn create_workspace(
    State(app_state): State<AppState>,
    Json(body): Json<WorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>, AppError> {
    let workspace = workspace::ActiveModel {
        name: Set(body.name),
        owner_id: Set(body.owner_id),
        ..Default::default()
    };
    let workspace = workspace.insert(&*app_state.db).await?;
    Ok(Json(WorkspaceResponse::from(workspace)))
}

#[utoipa::path(
    get,
    path = "/workspaces/{user_id}",
    tag = "workspace",
    responses((status = 200, body = inline(Vec<WorkspaceResponse>)))
)]
async fn get_workspaces(
    State(app_state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<WorkspaceResponse>>, AppError> {
    let workspaces = workspace::Entity::find()
        .filter(workspace::Column::OwnerId.eq(user_id))
        .all(&*app_state.db)
        .await?;
    Ok(Json(
        workspaces
            .into_iter()
            .map(WorkspaceResponse::from)
            .collect(),
    ))
}

#[utoipa::path(
    get,
    path = "/workspace/{workspace_id}",
    tag = "workspace",
    responses((status = 200, body = inline(Vec<WorkspaceResponse>)))
)]
async fn get_workspace(
    State(app_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<WorkspaceResponse>, AppError> {
    let workspace = workspace::Entity::find()
        .filter(workspace::Column::Id.eq(workspace_id))
        .one(&*app_state.db)
        .await?
        .ok_or(AppError::NotFound("Workspace not found".to_string()))?;

    Ok(Json(WorkspaceResponse::from(workspace)))
}

#[utoipa::path(
    delete,
    path = "/workspace/{workspace_id}",
    tag = "workspace",
    responses((status = 200, body = DeleteResponse))
)]
async fn delete_workspace(
    State(app_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<DeleteResponse>, AppError> {
    let result = workspace::Entity::delete_by_id(workspace_id)
        .exec(&*app_state.db)
        .await?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound("Workspace not found".to_string()));
    }

    Ok(Json(DeleteResponse {
        rows_affected: result.rows_affected,
    }))
}

#[utoipa::path(
    put,
    path = "/workspace",
    tag = "workspace",
    request_body = RenameRequest,
    responses((status = 200, body = WorkspaceResponse))
)]
async fn rename_workspace(
    State(app_state): State<AppState>,
    Json(body): Json<RenameRequest>,
) -> Result<Json<WorkspaceResponse>, AppError> {
    let mut workspace = workspace::Entity::find_by_id(body.id)
        .one(&*app_state.db)
        .await?
        .ok_or(AppError::NotFound("Workspace not found".to_string()))?
        .into_active_model();

    workspace.name = Set(body.name);
    let workspace = workspace.update(&*app_state.db).await?;
    Ok(Json(WorkspaceResponse::from(workspace)))
}
