use backend::dto::workspace::{DeleteResponse, RenameRequest, WorkspaceRequest, WorkspaceResponse};
use eyre::Result;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use serde_json::json;
use uuid::Uuid;
mod common;
use crate::common::{helpers::mock_workspace, server::create_test_app};
#[tokio::test]
async fn get_workspaces() -> Result<()> {
    let id = Uuid::new_v4();
    let id2 = Uuid::new_v4();
    let user_id = "user_test_nod_prod";
    let mock_data = [
        mock_workspace(id, "test_1", user_id),
        mock_workspace(id2, "test_2", user_id),
    ];

    let mock_db =
        MockDatabase::new(DatabaseBackend::Postgres).append_query_results([mock_data.clone()]);

    let server = create_test_app(mock_db).await?;
    let response = server
        .get(format!("/workspaces/{}", user_id).as_str())
        .await;

    response.assert_status_ok();
    response.assert_json(&mock_data);
    Ok(())
}
#[tokio::test]
async fn create_workspace() -> Result<()> {
    let id = Uuid::new_v4();
    let user_id = "user_test_nod_prod";
    let name = "test1";
    let expected = mock_workspace(id, name, user_id);
    let mock_db =
        MockDatabase::new(DatabaseBackend::Postgres).append_query_results([[expected.clone()]]);
    let server = create_test_app(mock_db).await?;
    let response = server
        .post("/workspace")
        .json(&json!(WorkspaceRequest {
            name: name.to_string(),
            owner_id: user_id.to_string(),
        }))
        .await;
    response.assert_status_ok();
    response.assert_json(&expected);
    Ok(())
}
#[tokio::test]
async fn delete_workspace() -> Result<()> {
    let id = Uuid::new_v4();
    let user_id = "user_test_nod_prod";
    let mock_data = [mock_workspace(id, "test_1", user_id)];
    let mock_db =
        MockDatabase::new(DatabaseBackend::Postgres).append_exec_results([MockExecResult {
            rows_affected: 1,
            last_insert_id: 0, // Not used, but required by the struct
        }]);
    let server = create_test_app(mock_db).await?;
    let response = server
        .delete(format!("/workspace?workspace_id={}", id).as_str())
        .await;
    response.assert_status_ok();
    let json: DeleteResponse = response.json();
    println!("{:?}", json);
    let expected = DeleteResponse {
        rows_affected: mock_data.len() as u64,
    };
    response.assert_json(&expected);
    Ok(())
}
#[tokio::test]
async fn rename_workspaces() -> Result<()> {
    let id = Uuid::new_v4();
    let user_id = "user_test_nod_prod";
    let rename = "update_test_1";
    let mock_data = mock_workspace(id, "test_1", user_id);
    let expected = mock_workspace(id, rename, user_id);
    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[mock_data.clone()]])
        .append_query_results([[expected.clone()]]);
    let server = create_test_app(mock_db).await?;
    let response = server
        .put(format!("/workspace/{}", id).as_str())
        .json(&json!(RenameRequest {
            id,
            name: rename.to_string()
        }))
        .await;
    let expected = WorkspaceResponse::from(expected);
    response.assert_status_ok();
    response.assert_json(&expected);
    Ok(())
}
