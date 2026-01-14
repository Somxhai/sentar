use crate::app::AppState;
use crate::dto::workspace::{DeleteResponse, RenameRequest, WorkspaceRequest, WorkspaceResponse};
use crate::error::AppError;
use crate::model::workspace;
use axum::extract::{Path, Query};
use axum::{Json, extract::State};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use std::collections::HashMap;
use std::iter::Iterator;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

pub fn workspace_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(
        create_workspace,
        delete_workspace,
        rename_workspace,
        get_workspace
    ))
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
        id: Set(Uuid::new_v4()),
        name: Set(body.name),
        owner_id: Set(body.owner_id),
        ..Default::default()
    };
    let workspace = workspace.insert(&*app_state.db).await?;
    Ok(Json(WorkspaceResponse::from(workspace)))
}

pub mod workspaces {
    use super::*;

    pub fn workspaces_routes() -> OpenApiRouter<AppState> {
        OpenApiRouter::new().routes(routes!(get_workspaces))
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
            .await?
            .into_iter()
            .map(WorkspaceResponse::from)
            .collect();
        Ok(Json(workspaces))
    }
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
    path = "/workspace",
    tag = "workspace",
    params(
        ("workspace_id" = Uuid, Query, description = "ID of the workspace to delete")
    ),
    responses((status = 200, body = DeleteResponse))
)]
async fn delete_workspace(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<DeleteResponse>, AppError> {
    let workspace_id: Uuid = params
        .get("workspace_id")
        .ok_or(AppError::BadRequest(
            "Missing `workspace_id` query parameter".to_string(),
        ))?
        .parse()
        .map_err(|_| AppError::BadRequest("Invalid UUID for `workspace_id`".to_string()))?;

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
