use crate::app::AppState;
use crate::dto::cache::SessionCache;
use crate::dto::connection_info::ConnectionInfo;
use crate::dto::form::submission::{FormSubmit, FormSubmitResponse};
use crate::error::AppError;
use axum::Extension;
use axum::response::{IntoResponse, Response};
use axum::{
    Json,
    extract::{Path, State},
};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

pub fn form_submission_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(submit_form))
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/form/{form_id}/submission",
    description="Use when user submitting form in the event which can submit or draft.",
    tags = ["form"],
    params(
        ("form_id" = Uuid,  Path, description = "ID of the form to delete")
    ),
    responses((status = 201, body = FormSubmitResponse))
)]
async fn submit_form(
    Extension(user_cache): Extension<SessionCache>,
    Extension(connection_info): Extension<ConnectionInfo>,
    State(app_state): State<AppState>,
    Path(form_id): Path<Uuid>,
    Json(body): Json<FormSubmit>,
) -> Result<Response, AppError> {
    let user_id = user_cache.user_id;
    let db = &*app_state.db;
    let submission = body
        .submit(db, user_id, form_id, connection_info)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(FormSubmitResponse::from(submission).into_response())
}
