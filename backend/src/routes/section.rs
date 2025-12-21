use std::collections::HashMap;

use crate::app::AppState;
use crate::dto::section::{SectionRequest, SectionResponse, UpdateSectionRequest};
use crate::dto::workspace::DeleteResponse;
use crate::error::AppError;
use crate::model::section;
use axum::extract::Query;
use axum::{
    Json,
    extract::{Path, State},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

pub fn section_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(
        get_section,
        create_section,
        delete_section,
        update_section
    ))
}

#[utoipa::path(
    get,
    path = "/section/{id}",
    tag = "section",
    responses((status = 200, body = SectionResponse))
)]
pub async fn get_section(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SectionResponse>, AppError> {
    let section = section::Entity::find_by_id(id)
        .one(&*app_state.db)
        .await?
        .ok_or(AppError::NotFound("Section not found".to_string()))?;
    Ok(Json(SectionResponse::from(section)))
}

#[utoipa::path(
    post,
    path = "/section",
    tag = "section",
    request_body = SectionRequest,
    responses((status = 200, body = SectionResponse))
)]
async fn create_section(
    State(app_state): State<AppState>,
    Json(body): Json<SectionRequest>,
) -> Result<Json<SectionResponse>, AppError> {
    let section = section::ActiveModel {
        event_id: Set(body.event_id),
        title: Set(body.title),
        price: Set(body.price),
        ..Default::default()
    };

    let section = section.insert(&*app_state.db).await?;
    Ok(Json(SectionResponse::from(section)))
}

#[utoipa::path(
    delete,
    path = "/section",
    tag = "section",
    params(
    ("section_id" = Uuid, Query),
    ),
    responses((status = 200, body = DeleteResponse))
)]
async fn delete_section(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<DeleteResponse>, AppError> {
    let section_id: Uuid = params
        .get("id")
        .ok_or(AppError::BadRequest(
            "Missing `id` query parameter".to_string(),
        ))?
        .parse()
        .map_err(|_| AppError::BadRequest("Invalid UUID for `id`".to_string()))?;
    let result = section::Entity::delete_by_id(section_id)
        .exec(&*app_state.db)
        .await?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound("Section not found".to_string()));
    }

    Ok(Json(DeleteResponse {
        rows_affected: result.rows_affected,
    }))
}

#[utoipa::path(
    put,
    path = "/section",
    tag = "section",
    request_body = UpdateSectionRequest,
    responses((status = 200, body = SectionResponse))
)]
async fn update_section(
    State(app_state): State<AppState>,
    Json(body): Json<UpdateSectionRequest>,
) -> Result<Json<SectionResponse>, AppError> {
    let mut section = section::Entity::find_by_id(body.id)
        .one(&*app_state.db)
        .await?
        .ok_or(AppError::NotFound("Section not found".to_string()))?
        .into_active_model();

    if let Some(title) = body.title {
        section.title = Set(title);
    }
    if let Some(price) = body.price {
        section.price = Set(price);
    }

    let updated_section = section.update(&*app_state.db).await?;
    Ok(Json(SectionResponse::from(updated_section)))
}
