use axum_test::TestServer;
use backend::dto::event::{EventRequest, EventResponse, UpdateEventRequest};
use backend::dto::workspace::DeleteResponse;
use eyre::Result;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use serde_json::json;
use uuid::Uuid;

mod common;
use crate::common::helpers::{create_test_app, mock_event};

#[tokio::test]
async fn get_event() -> Result<()> {
    let id = Uuid::new_v4();
    let workspace_id = Uuid::new_v4();
    let title = "Test Event";

    let mock_data = mock_event(id, title, workspace_id);

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![mock_data.clone()]]);

    let app = create_test_app(mock_db).await?;
    let server = TestServer::new(app).unwrap();

    let response = server.get(format!("/event/{}", id).as_str()).await;

    response.assert_status_ok();
    let json: EventResponse = response.json();
    assert_eq!(json.id, id);

    Ok(())
}

#[tokio::test]
async fn create_event() -> Result<()> {
    let id = Uuid::new_v4();
    let workspace_id = Uuid::new_v4();
    let title = "Test Event";

    let expected = mock_event(id, title, workspace_id);

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![expected.clone()]]);

    let app = create_test_app(mock_db).await?;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/event")
        .json(&json!(EventRequest {
            title: title.to_string(),
            workspace_id,
            description: None,
            starts_at: None,
            ends_at: None,
            settings: None,
        }))
        .await;

    response.assert_status_ok();
    let json: EventResponse = response.json();
    assert_eq!(json.id, id);
    Ok(())
}

#[tokio::test]
async fn delete_event() -> Result<()> {
    let id = Uuid::new_v4();

    let mock_db =
        MockDatabase::new(DatabaseBackend::Postgres).append_exec_results(vec![MockExecResult {
            rows_affected: 1,
            last_insert_id: 0,
        }]);

    let app = create_test_app(mock_db).await?;
    let server = TestServer::new(app).unwrap();

    let response = server
        .delete(format!("/event?event_id={}", id).as_str())
        .await;

    response.assert_status_ok();
    let expected = DeleteResponse { rows_affected: 1 };
    response.assert_json(&expected);
    Ok(())
}

#[tokio::test]
async fn update_event() -> Result<()> {
    let id = Uuid::new_v4();
    let workspace_id = Uuid::new_v4();
    let old_title = "Old Event";
    let new_title = "Updated Event";

    let mock_old = mock_event(id, old_title, workspace_id);
    let mock_new = mock_event(id, new_title, workspace_id);

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![mock_old.clone()]])
        .append_query_results(vec![vec![mock_new.clone()]]);

    let app = create_test_app(mock_db).await?;
    let server = TestServer::new(app).unwrap();

    let response = server
        .put("/event")
        .json(&json!(UpdateEventRequest {
            id,
            title: Some(new_title.to_string()),
            description: None,
            starts_at: None,
            ends_at: None,
            settings: None,
        }))
        .await;

    response.assert_status_ok();
    let json: EventResponse = response.json();
    assert_eq!(json.id, id);
    assert_eq!(json.title, new_title);
    Ok(())
}
