#[allow(dead_code)]
pub mod helpers {

    use axum::Router;
    use backend::{
        app::{cache::create_cache, create_router},
        model::{event, form, section, workspace},
    };
    use chrono::{DateTime, NaiveDateTime};
    use eyre::Result;
    use sea_orm::MockDatabase;
    use uuid::Uuid;

    pub async fn create_test_app(mock_db: MockDatabase) -> Result<Router> {
        let db = mock_db.into_connection();
        let cache = create_cache().await?;
        Ok(create_router(db, cache)?)
    }

    pub fn mock_datetime() -> NaiveDateTime {
        DateTime::from_timestamp(1700000000, 0).unwrap().naive_utc()
    }

    pub fn mock_workspace(id: Uuid, name: &str, owner_id: &str) -> workspace::Model {
        let now = mock_datetime();
        workspace::Model {
            id,
            name: name.to_string(),
            owner_id: owner_id.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
    pub fn mock_section(id: Uuid, title: &str, event_id: Uuid, price: f64) -> section::Model {
        let now = mock_datetime();
        section::Model {
            id,
            event_id,
            title: title.to_string(),
            price,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mock_event(id: Uuid, title: &str, workspace_id: Uuid) -> event::Model {
        let now = mock_datetime();
        event::Model {
            id,
            workspace_id,
            title: title.to_string(),
            description: None,
            starts_at: None,
            ends_at: None,
            settings: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mock_form(id: Uuid, event_id: Uuid, title: &str, description: &str) -> form::Model {
        let now = mock_datetime();
        form::Model {
            id,
            event_id,
            title: Some(title.to_string()),
            description: Some(description.to_string()),
            schema: None,
            settings: None,
            created_at: now,
            updated_at: now,
        }
    }
}
