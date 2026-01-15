use backend::dto::event::{EventRequest, EventResponse, UpdateEventRequest};
use uuid::Uuid;

use crate::common::{container::PgContainer, seeding::Seeding, server::create_test_server};

mod common;

#[tokio::test]
async fn create_event() -> eyre::Result<()> {
    let db = PgContainer::create_test_db().await?;
    let user_id = "my_user_id";
    let app = create_test_server(&db, user_id).await?;

    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding
        .create_workspace("test_workspace", user_id.into())
        .await;

    let response = app
        .post("/event")
        .json(&EventRequest {
            title: format!("test_event_{}", Uuid::new_v4()),
            workspace_id: workspace.id,
            ..Default::default()
        })
        .await;

    response.assert_status_ok();

    db.close().await.expect("Failed to close db");
    Ok(())
}

#[tokio::test]
async fn update_event() -> eyre::Result<()> {
    let db = PgContainer::create_test_db().await?;
    let user_id = "my_user_id";
    let app = create_test_server(&db, user_id).await?;

    let seeding = Seeding(&db);

    let user_id = "my_user_id";

    seeding.create_user(user_id.into()).await;
    let workspace = seeding
        .create_workspace("test_workspace", user_id.into())
        .await;

    let response = app
        .post("/event")
        .json(&EventRequest {
            title: format!("test_event_{}", Uuid::new_v4()),
            workspace_id: workspace.id,
            ..Default::default()
        })
        .await;

    response.assert_status_ok();

    let json: EventResponse = response.json();
    let new_title = format!("updated_test_event_{}", Uuid::new_v4());
    let response = app
        .put("/event")
        .json(&UpdateEventRequest {
            id: json.id,
            title: Some(new_title.clone()),
            ..Default::default()
        })
        .await;

    response.assert_status_ok();

    let json: EventResponse = response.json();

    assert_eq!(json.title, new_title);

    db.close().await.expect("Failed to close db");
    Ok(())
}

#[tokio::test]
async fn get_event() -> eyre::Result<()> {
    let db = PgContainer::create_test_db().await?;
    let user_id = "my_user_id";
    let app = create_test_server(&db, user_id).await?;

    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding
        .create_workspace("test_workspace", user_id.into())
        .await;

    let create_res = app
        .post("/event")
        .json(&EventRequest {
            title: format!("test_event_{}", Uuid::new_v4()),
            workspace_id: workspace.id,
            ..Default::default()
        })
        .await;

    let created_event: EventResponse = create_res.json();

    let response = app.get(&format!("/event/{}", created_event.id)).await;

    response.assert_status_ok();
    let json: EventResponse = response.json();
    assert_eq!(json.id, created_event.id);
    assert_eq!(json.title, created_event.title);

    db.close().await.expect("Failed to close db");
    Ok(())
}

#[tokio::test]
async fn delete_event() -> eyre::Result<()> {
    use backend::dto::workspace::DeleteResponse;

    let user_id = "my_user_id";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;

    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding
        .create_workspace("test_workspace", user_id.into())
        .await;

    let create_res = app
        .post("/event")
        .json(&EventRequest {
            title: format!("test_event_{}", Uuid::new_v4()),
            workspace_id: workspace.id,
            ..Default::default()
        })
        .await;

    let created_event: EventResponse = create_res.json();

    let response = app
        .delete(&format!("/event?event_id={}", created_event.id))
        .await;

    response.assert_status_ok();

    let json: DeleteResponse = response.json();
    assert_eq!(json.rows_affected, 1);

    db.close().await.expect("Failed to close db");
    Ok(())
}
