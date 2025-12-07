use crate::app::AppState;
use crate::dto::event::{EventRequest, EventResponse, UpdateEventRequest};
use crate::dto::workspace::DeleteResponse;
use crate::error::AppError;
use crate::model::event;
use axum::extract::{Path, Query};
use axum::{Json, extract::State};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use std::collections::HashMap;
use std::iter::Iterator;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

pub fn event_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_event, create_event, delete_event, update_event))
}

#[utoipa::path(
    get,
    path = "/event/{id}",
    tag = "event",
    responses((status = 200, body = EventResponse))
)]
pub async fn get_event(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EventResponse>, AppError> {
    let event = event::Entity::find_by_id(id)
        .one(&*app_state.db)
        .await?
        .ok_or(AppError::NotFound("Event not found".to_string()))?;
    Ok(Json(EventResponse::from(event)))
}

#[utoipa::path(
    post,
    path = "/event",
    tag = "event",
    request_body = EventRequest,
    responses((status = 200, body = EventResponse))
)]
async fn create_event(
    State(app_state): State<AppState>,
    Json(body): Json<EventRequest>,
) -> Result<Json<EventResponse>, AppError> {
    let event = event::ActiveModel {
        title: Set(body.title),
        workspace_id: Set(body.workspace_id),
        description: Set(body.description),
        starts_at: Set(body.starts_at),
        ends_at: Set(body.ends_at),
        settings: Set(body.settings),
        ..Default::default()
    };
    let event = event.insert(&*app_state.db).await?;
    Ok(Json(EventResponse::from(event)))
}

#[utoipa::path(
    delete,
    path = "/event",
    tag = "event",
    params(
        ("event_id" = Uuid, Query, description = "ID of the event to delete")
    ),
    responses((status = 200, body = DeleteResponse))
)]
async fn delete_event(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<DeleteResponse>, AppError> {
    let event_id: Uuid = params
        .get("event_id")
        .ok_or(AppError::BadRequest(
            "Missing `event_id` query parameter".to_string(),
        ))?
        .parse()
        .map_err(|_| AppError::BadRequest("Invalid UUID for `event_id`".to_string()))?;

    let result = event::Entity::delete_by_id(event_id)
        .exec(&*app_state.db)
        .await?;

    Ok(Json(DeleteResponse {
        rows_affected: result.rows_affected,
    }))
}

#[utoipa::path(
    put,
    path = "/event",
    tag = "event",
    request_body = UpdateEventRequest,
    responses((status = 200, body = EventResponse))
)]
async fn update_event(
    State(app_state): State<AppState>,
    Json(body): Json<UpdateEventRequest>,
) -> Result<Json<EventResponse>, AppError> {
    let mut event = event::Entity::find_by_id(body.id)
        .one(&*app_state.db)
        .await?
        .ok_or(AppError::NotFound("Event not found".to_string()))?
        .into_active_model();

    if let Some(title) = body.title {
        event.title = Set(title);
    }
    if let Some(description) = body.description {
        event.description = Set(Some(description));
    }
    if let Some(starts_at) = body.starts_at {
        event.starts_at = Set(Some(starts_at));
    }
    if let Some(ends_at) = body.ends_at {
        event.ends_at = Set(Some(ends_at));
    }
    if let Some(settings) = body.settings {
        event.settings = Set(Some(settings));
    }

    let updated_event = event.update(&*app_state.db).await?;
    Ok(Json(EventResponse::from(updated_event)))
}
