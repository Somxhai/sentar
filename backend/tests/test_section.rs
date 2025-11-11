use axum_test::TestServer;
use backend::dto::section::{SectionRequest, SectionResponse, UpdateSectionRequest};
use backend::dto::workspace::DeleteResponse;
use eyre::Result;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use serde_json::json;
use uuid::Uuid;

mod common;
use crate::common::helpers::{create_test_app, mock_section};

#[tokio::test]
async fn get_section() -> Result<()> {
    let id = Uuid::new_v4();
    let event_id = Uuid::new_v4();
    let title = "Test Section";
    let price = 100.0;

    let mock_data = mock_section(id, title, event_id, price);

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![mock_data.clone()]]);

    let app = create_test_app(mock_db).await?;
    let server = TestServer::new(app).unwrap();

    let response = server.get(format!("/section/{}", id).as_str()).await;

    response.assert_status_ok();
    response.assert_json(&SectionResponse { section: mock_data });

    Ok(())
}
#[tokio::test]
async fn create_section() -> Result<()> {
    let id = Uuid::new_v4();
    let event_id = Uuid::new_v4();
    let title = "Test Section";
    let price = 100.0;

    let expected = mock_section(id, title, event_id, price);

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![expected.clone()]]);

    let app = create_test_app(mock_db).await?;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/section")
        .json(&json!(SectionRequest {
            event_id,
            title: title.to_string(),
            price,
        }))
        .await;

    response.assert_status_ok();
    response.assert_json(&SectionResponse { section: expected });
    Ok(())
}

#[tokio::test]
async fn delete_section() -> Result<()> {
    let id = Uuid::new_v4();

    let mock_db =
        MockDatabase::new(DatabaseBackend::Postgres).append_exec_results(vec![MockExecResult {
            rows_affected: 1,
            last_insert_id: 0,
        }]);

    let app = create_test_app(mock_db).await?;
    let server = TestServer::new(app).unwrap();

    let response = server.delete(format!("/section?id={}", id).as_str()).await;

    response.assert_status_ok();
    let expected = DeleteResponse { rows_affected: 1 };
    response.assert_json(&expected);
    Ok(())
}

#[tokio::test]
async fn update_section() -> Result<()> {
    let id = Uuid::new_v4();
    let event_id = Uuid::new_v4();
    let old_title = "Old Section";
    let new_title = "Updated Section";
    let price = 50.0;

    let mock_old = mock_section(id, old_title, event_id, price);
    let mock_new = mock_section(id, new_title, event_id, price);

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![mock_old.clone()]])
        .append_query_results(vec![vec![mock_new.clone()]]);

    let app = create_test_app(mock_db).await?;
    let server = TestServer::new(app).unwrap();

    let response = server
        .put("/section")
        .json(&json!(UpdateSectionRequest {
            id,
            title: Some(new_title.to_string()),
            price: Some(price),
        }))
        .await;

    response.assert_status_ok();
    response.assert_json(&SectionResponse { section: mock_new });
    Ok(())
}
