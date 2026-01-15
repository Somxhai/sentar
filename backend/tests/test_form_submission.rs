use backend::dto::form::{
    FormRequest, FormResponse,
    submission::{FormSubmit, FormSubmitResponse},
};
use eyre::Result;
use serde_json::json;

use crate::common::{container::PgContainer, seeding::Seeding, server::create_test_server};
mod common;

#[tokio::test]
async fn submit_form() -> Result<()> {
    let user_id = "my-test-user";
    let db = PgContainer::create_test_db().await?;
    let app = create_test_server(&db, user_id).await?;
    let seeding = Seeding(&db);

    seeding.create_user(user_id.into()).await;
    let workspace = seeding.create_workspace("test_workspace", user_id).await;
    let event = seeding.create_event("test_event", workspace.id).await;

    // Create Form first
    let title = "Test Form";
    let description = "Test Description";
    let create_response = app
        .post("/form")
        .json(&json!(FormRequest {
            event_id: event.id,
            title: Some(title.to_string()),
            description: Some(description.to_string()),
            schema: None,
            settings: None,
        }))
        .await;
    create_response.assert_status_success();
    let form_json = create_response.json::<FormResponse>();
    println!("created form:\n{:?} ", form_json);

    // Prepare Submission
    let answers = json!([
        {
            "type": "textbox",
            "hint": "Type your username",
            "optional": true,
            "value": "my_username"
        }
    ]);

    let url = format!("/form/{}/submission", form_json.id);
    let body = FormSubmit {
        answers: answers.clone(),
        submission_id: None,
        is_draft: false,
    };

    let submit = app.post(&url).json(&body).await;
    submit.assert_status_success();

    let submit_result = submit.json::<FormSubmitResponse>();
    assert_eq!(submit_result.form_id, form_json.id);
    assert!(!submit_result.id.is_nil());

    Ok(())
}
