use backend::{
    dto::form::{
        FormRequest, FormResponse,
        submission::{FormSubmit, FormSubmitResponse},
    },
    model::form_submission,
};
use eyre::Result;
use sea_orm::{DatabaseBackend, MockDatabase};
use serde_json::json;
use uuid::Uuid;

use crate::common::{helpers::mock_form, server::create_test_app};
mod common;

#[tokio::test]
async fn submit_form() -> Result<()> {
    let id = Uuid::new_v4();
    let event_id = Uuid::new_v4();
    let title = "Test Form";
    let user_id = "my-test-user";
    let description = "Test Description";
    let expected_form = mock_form(id, event_id, title, description, user_id);
    let submission_id = Uuid::new_v4();

    let answers = json!([
        {
            "type": "textbox",
            "hint": "Type your username",
            "optional": true,
        }
    ]);

    let expected_submission = form_submission::Model {
        id: submission_id,
        form_id: id,
        respondent_id: user_id.to_string(),
        answer: answers.clone(),
        status: "submitted".to_owned(),
        updated_at: chrono::Utc::now().naive_utc(),
        submitted_at: chrono::Utc::now().naive_utc(),
        user_agent: None,
        ip_address: None,
    };

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[expected_form.clone()]])
        .append_query_results([[expected_submission.clone()]]);

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
    let form_json = response.json::<FormResponse>();

    println!("form_id: {}", form_json.id);
    let url = format!("/form/{}/submission", form_json.id);
    let body = FormSubmit {
        answers: answers.clone(),
        submission_id: None,
        is_draft: false,
    };

    let submit = server.post(&url).json(&body).await;
    submit.assert_status_success();

    let submit_result = submit.json::<FormSubmitResponse>();
    assert_eq!(submit_result.form_id, form_json.id);
    assert_eq!(submit_result.id, submission_id);
    Ok(())
}
