use axum::Router;
use backend::{app::create_router, model::workspace};
use chrono::{DateTime, NaiveDateTime};
use eyre::Result;
use sea_orm::MockDatabase;
use uuid::Uuid;

pub async fn create_test_app(mock_db: MockDatabase) -> Result<Router> {
    let db = mock_db.into_connection();
    Ok(create_router(db)?)
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
