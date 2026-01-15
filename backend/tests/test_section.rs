use backend::dto::section::{SectionRequest, SectionResponse, UpdateSectionRequest};
use backend::dto::workspace::DeleteResponse;
use eyre::Result;
use serde_json::json;
use uuid::Uuid;

use crate::common::{container::PgContainer, seeding::Seeding, server::create_test_server};
mod common;

#[tokio::test]
async fn get_section() -> Result<()> {
    let user_id = "user_test";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("ws", user_id).await;
    let event = seeding.create_event("evt", workspace.id).await;

    let section_id = Uuid::new_v4();
    let title = "Test Section";
    let price = 100.0;

    seeding
        .create_section(section_id, title, event.id, price)
        .await;

    let response = app.get(&format!("/section/{}", section_id)).await;

    response.assert_status_ok();
    let json: SectionResponse = response.json();
    assert_eq!(json.id, section_id);
    assert_eq!(json.title, title);
    assert_eq!(json.price, price);

    Ok(())
}

#[tokio::test]
async fn create_section() -> Result<()> {
    let user_id = "user_test";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("ws", user_id).await;
    let event = seeding.create_event("evt", workspace.id).await;

    let title = "Test Section";
    let price = 100.0;

    let response = app
        .post("/section")
        .json(&json!(SectionRequest {
            event_id: event.id,
            title: title.to_string(),
            price,
        }))
        .await;

    response.assert_status_ok();
    let json: SectionResponse = response.json();
    assert_eq!(json.title, title);
    assert_eq!(json.event_id, event.id);

    Ok(())
}

#[tokio::test]
async fn delete_section() -> Result<()> {
    let user_id = "user_test";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("ws", user_id).await;
    let event = seeding.create_event("evt", workspace.id).await;

    let section_id = Uuid::new_v4();
    seeding
        .create_section(section_id, "Delete Me", event.id, 50.0)
        .await;

    let response = app.delete(&format!("/section?id={}", section_id)).await;

    response.assert_status_ok();
    let expected = DeleteResponse { rows_affected: 1 };
    response.assert_json(&expected);
    Ok(())
}

#[tokio::test]
async fn update_section() -> Result<()> {
    let user_id = "user_test";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("ws", user_id).await;
    let event = seeding.create_event("evt", workspace.id).await;

    let section_id = Uuid::new_v4();
    seeding
        .create_section(section_id, "Old Section", event.id, 50.0)
        .await;

    let new_title = "Updated Section";
    let price = 50.0;

    let response = app
        .put("/section")
        .json(&json!(UpdateSectionRequest {
            id: section_id,
            title: Some(new_title.to_string()),
            price: Some(price),
        }))
        .await;

    response.assert_status_ok();
    let json: SectionResponse = response.json();
    assert_eq!(json.id, section_id);
    assert_eq!(json.title, new_title);

    Ok(())
}
