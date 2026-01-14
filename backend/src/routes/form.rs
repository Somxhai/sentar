use crate::app::AppState;
use crate::dto::cache::SessionCache;
use crate::dto::form::{FormRequest, FormResponse, UpdateFormRequest};
use crate::dto::workspace::DeleteResponse;
use crate::error::AppError;
use crate::model::form;
use axum::Extension;
use axum::response::{IntoResponse, Response};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};
use std::collections::HashMap;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

pub fn form_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(create_form, update_form, delete_form))
}

pub fn public_form_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_form))
}

#[utoipa::path(
    post,
    path = "/form",
    tag = "form",
    request_body = FormRequest,
    responses((status = 201, body = FormResponse))
)]
async fn create_form(
    State(app_state): State<AppState>,
    Extension(extension): Extension<SessionCache>,
    Json(body): Json<FormRequest>,
) -> Result<Response, AppError> {
    let form = form::ActiveModel {
        id: Set(Uuid::new_v4()),
        event_id: Set(body.event_id),
        title: Set(body.title),
        description: Set(body.description),
        schema: Set(body.schema),
        settings: Set(body.settings),
        is_active: Set(true),
        updated_by: Set(extension.user_id),
        ..Default::default()
    };

    let form = form.insert(&*app_state.db).await?;
    Ok(FormResponse::from(form).into_response())
}

#[utoipa::path(
    get,
    path = "/form/{form_id}",
    tag = "form",
    responses((status = 200, body = FormResponse))
)]
async fn get_form(
    State(app_state): State<AppState>,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormResponse>, AppError> {
    let form = form::Entity::find_by_id(form_id)
        .one(&*app_state.db)
        .await?
        .ok_or(AppError::NotFound("Form not found".to_string()))?;

    Ok(Json(form.into()))
}

#[utoipa::path(
    patch,
    path = "/form",
    tag = "form",
    request_body = UpdateFormRequest,
    responses((status = 200, body = FormResponse))
)]
async fn update_form(
    State(app_state): State<AppState>,
    Json(body): Json<UpdateFormRequest>,
) -> Result<Json<FormResponse>, AppError> {
    let mut form = form::Entity::find_by_id(body.id)
        .one(&*app_state.db)
        .await?
        .ok_or(AppError::NotFound("Form not found".to_string()))?
        .into_active_model();

    if let Some(title) = body.title {
        form.title = Set(Some(title));
    }
    if let Some(description) = body.description {
        form.description = Set(Some(description));
    }
    if let Some(schema) = body.schema {
        form.schema = Set(Some(schema));
    }
    if let Some(settings) = body.settings {
        form.settings = Set(Some(settings));
    }

    let updated_form = form.update(&*app_state.db).await?;
    Ok(Json(updated_form.into()))
}

#[utoipa::path(
    delete,
    path = "/form",
    tag = "form",
    params(
        ("form_id" = Uuid, Query, description = "ID of the form to delete")
    ),
    responses((status = 200, body = DeleteResponse))
)]
async fn delete_form(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<DeleteResponse>, AppError> {
    let form_id: Uuid = params
        .get("form_id")
        .ok_or(AppError::BadRequest(
            "Missing `form_id` query parameter".to_string(),
        ))?
        .parse()
        .map_err(|_| AppError::BadRequest("Invalid UUID for `form_id`".to_string()))?;

    let result = form::Entity::delete_by_id(form_id)
        .exec(&*app_state.db)
        .await?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound("Form not found".to_string()));
    }

    Ok(Json(DeleteResponse {
        rows_affected: result.rows_affected,
    }))
}
