use backend::dto::workspace::{DeleteResponse, RenameRequest, WorkspaceRequest, WorkspaceResponse};
use eyre::Result;
use serde_json::json;
use uuid::Uuid;

use crate::common::{container::PgContainer, seeding::Seeding, server::create_test_server};
mod common;

#[tokio::test]
async fn get_workspaces() -> Result<()> {
    let user_id = "user_test_nod_prod";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;

    seeding.create_workspace("test_1", user_id).await;
    seeding.create_workspace("test_2", user_id).await;

    let response = app.get(&format!("/workspaces/{}", user_id)).await;

    response.assert_status_ok();
    let json: Vec<WorkspaceResponse> = response.json();
    assert_eq!(json.len(), 2);
    Ok(())
}

#[tokio::test]
async fn create_workspace() -> Result<()> {
    let user_id = "user_test_nod_prod";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;

    let name = "test1";

    let response = app
        .post("/workspace")
        .json(&json!(WorkspaceRequest {
            name: name.to_string(),
            owner_id: user_id.to_string(),
        }))
        .await;

    response.assert_status_ok();
    let json: WorkspaceResponse = response.json();
    assert_eq!(json.name, name);
    assert_eq!(json.owner_id, user_id);
    Ok(())
}

#[tokio::test]
async fn delete_workspace() -> Result<()> {
    let user_id = "user_test_nod_prod";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("test_1", user_id).await;

    let response = app
        .delete(&format!("/workspace?workspace_id={}", workspace.id))
        .await;

    response.assert_status_ok();
    let expected = DeleteResponse { rows_affected: 1 };
    response.assert_json(&expected);
    Ok(())
}

#[tokio::test]
async fn rename_workspaces() -> Result<()> {
    let user_id = "user_test_nod_prod";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("test_1", user_id).await;

    let rename = "update_test_1";

    let response = app
        .put(&format!("/workspace/{}", workspace.id))
        .json(&json!(RenameRequest {
            id: workspace.id,
            name: rename.to_string()
        }))
        .await;

    response.assert_status_ok();
    let json: WorkspaceResponse = response.json();
    assert_eq!(json.id, workspace.id);
    assert_eq!(json.name, rename);
    Ok(())
}
