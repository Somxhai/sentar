use crate::common::helpers::mock_form;
use crate::common::server::create_test_app;
use backend::dto::form::{FormRequest, FormResponse, UpdateFormRequest};
use backend::dto::workspace::DeleteResponse;
use eyre::Result;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use serde_json::json;
use uuid::Uuid;
mod common;

#[tokio::test]
async fn get_form() -> Result<()> {
    let id = Uuid::new_v4();
    let user_id = "my-test-user";
    let event_id = Uuid::new_v4();
    let title = "Test Form";
    let description = "Test Description";
    let mock_data = mock_form(id, event_id, title, description, user_id);
    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![mock_data.clone()]]);
    let server = create_test_app(mock_db).await?;
    let response = server.get(format!("/form/{}", id).as_str()).await;
    response.assert_status_ok();
    response.assert_json(&FormResponse::from(mock_data));
    Ok(())
}

#[tokio::test]
async fn create_form() -> Result<()> {
    let id = Uuid::new_v4();
    let event_id = Uuid::new_v4();
    let title = "Test Form";
    let user_id = "my-test-user";
    let description = "Test Description";
    let expected = mock_form(id, event_id, title, description, user_id);
    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![expected.clone()]]);
    let server = create_test_app(mock_db).await?;
    let response = server
        .post("/form")
        .json(&json!(FormRequest {
            event_id,
            title: Some(title.to_string()),
            description: Some(description.to_string()),
            schema: None,
            settings: None,
        }))
        .await;
    response.assert_status_success();
    response.assert_json(&FormResponse::from(expected));
    Ok(())
}

#[tokio::test]
async fn delete_form() -> Result<()> {
    let id = Uuid::new_v4();
    let mock_db =
        MockDatabase::new(DatabaseBackend::Postgres).append_exec_results(vec![MockExecResult {
            rows_affected: 1,
            last_insert_id: 0,
        }]);
    let server = create_test_app(mock_db).await?;
    let response = server
        .delete(format!("/form?form_id={}", id).as_str())
        .await;
    response.assert_status_ok();
    let expected = DeleteResponse { rows_affected: 1 };
    response.assert_json(&expected);
    Ok(())
}

#[tokio::test]
async fn update_form() -> Result<()> {
    let id = Uuid::new_v4();
    let event_id = Uuid::new_v4();
    let old_title = "Old Form";
    let new_title = "Updated Form";
    let description = "Test Description";
    let user_id = "my-test-user";
    let new_user_id = "my-new-user-id";
    let mock_old = mock_form(id, event_id, old_title, description, user_id);
    let mock_new = mock_form(id, event_id, new_title, description, new_user_id);
    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![mock_old.clone()]])
        .append_query_results(vec![vec![mock_new.clone()]]);
    let server = create_test_app(mock_db).await?;
    let response = server
        .patch("/form")
        .json(&json!(UpdateFormRequest {
            id,
            title: Some(new_title.to_string()),
            description: Some(description.to_string()),
            schema: None,
            settings: None,
        }))
        .await;
    response.assert_status_ok();
    let data = FormResponse::from(mock_new);

    response.assert_json(&data);

    assert_eq!(
        data.updated_by, new_user_id,
        "Updated user id does not match with new_user_id"
    );
    Ok(())
}
