use crate::common::{container::PgContainer, seeding::Seeding, server::create_test_server};
use backend::dto::form::{FormRequest, FormResponse, UpdateFormRequest};
use backend::dto::workspace::DeleteResponse;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn get_form() -> eyre::Result<()> {
    let user_id = "my_user_id";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("test_workspace", user_id).await;
    let event = seeding.create_event("test_event", workspace.id).await;

    let form_id = Uuid::new_v4();
    let title = "Test Form";
    let _form = seeding
        .create_form(form_id, event.id, title, "desc", user_id)
        .await;

    let response = app.get(&format!("/form/{}", form_id)).await;

    response.assert_status_ok();
    let json: FormResponse = response.json();
    assert_eq!(json.id, form_id);
    assert_eq!(json.title, Some(title.to_string()));

    Ok(())
}

#[tokio::test]
async fn create_form() -> eyre::Result<()> {
    let user_id = "my_user_id";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("test_workspace", user_id).await;
    let event = seeding.create_event("test_event", workspace.id).await;

    let title = "Test Form";
    let description = "Test Description";

    let response = app
        .post("/form")
        .json(&FormRequest {
            event_id: event.id,
            title: Some(title.to_string()),
            description: Some(description.to_string()),
            schema: None,
            settings: None,
        })
        .await;

    response.assert_status_success();
    let json: FormResponse = response.json();

    assert_eq!(json.event_id, event.id);
    assert_eq!(json.title, Some(title.to_string()));

    Ok(())
}

#[tokio::test]
async fn delete_form() -> eyre::Result<()> {
    let user_id = "my_user_id";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("test_workspace", user_id).await;
    let event = seeding.create_event("test_event", workspace.id).await;

    let form_id = Uuid::new_v4();
    let form = seeding
        .create_form(form_id, event.id, "To Delete", "desc", user_id)
        .await;

    let response = app.delete(&format!("/form?form_id={}", form.id)).await;

    response.assert_status_ok();
    let json: DeleteResponse = response.json();
    assert_eq!(json.rows_affected, 1);

    Ok(())
}

#[tokio::test]
async fn update_form() -> eyre::Result<()> {
    let user_id = "my_user_id";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("test_workspace", user_id).await;
    let event = seeding.create_event("test_event", workspace.id).await;

    let form_id = Uuid::new_v4();
    let form = seeding
        .create_form(form_id, event.id, "Old Form", "desc", user_id)
        .await;

    let new_title = "Updated Form";
    let response = app
        .patch("/form")
        .json(&UpdateFormRequest {
            id: form.id,
            title: Some(new_title.to_string()),
            description: Some("Updated Desc".to_string()),
            schema: None,
            settings: None,
        })
        .await;

    response.assert_status_ok();
    let json: FormResponse = response.json();
    assert_eq!(json.id, form.id);
    assert_eq!(json.title, Some(new_title.to_string()));

    Ok(())
}
