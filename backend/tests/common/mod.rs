pub mod layout;
pub mod reservation;
pub mod server;
#[allow(unused)]
pub mod helpers {

    use backend::model::{
        event, event_object, event_object_position, form, reservation, section, workspace,
        workspace_member,
    };
    use chrono::{DateTime, NaiveDateTime};
    use sea_orm::{ActiveValue::Set, TryIntoModel};
    use uuid::Uuid;

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

    pub fn mock_form(
        id: Uuid,
        event_id: Uuid,
        title: &str,
        description: &str,
        user_id: &str,
    ) -> form::Model {
        let now = mock_datetime();
        form::Model {
            id,
            event_id,
            title: Some(title.to_string()),
            description: Some(description.to_string()),
            schema: None,
            is_active: false,
            updated_by: user_id.to_string(),
            settings: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mock_reservation(user_id: String, event_id: Uuid) -> reservation::Model {
        let now = mock_datetime();

        reservation::Model {
            id: Uuid::new_v4(),
            user_id,
            event_id,
            status: "on_hold".to_string(),
            total_price: 0.0,
            expires_at: None,
            created_at: now,
            updated_at: now,
            approved_by: None,
            approved_at: None,
        }
    }

    pub fn mock_workspace_member(
        workspace_id: Uuid,
        user_id: String,
        invited_by: String,
    ) -> workspace_member::Model {
        let now = mock_datetime();
        workspace_member::Model {
            id: Uuid::new_v4(),
            workspace_id,
            user_id,
            status: "pending".into(),
            invited_by,
            role: "admin".into(),
            updated_at: now,
            created_at: now,
        }
    }

    pub fn mock_event_object(
        event_id: Uuid,
        section_id: Option<Uuid>,
        status: String,
    ) -> event_object::Model {
        let now = mock_datetime();
        event_object::Model {
            id: Uuid::new_v4(),
            object_type: "seat".into(), // or "table", "stage"
            event_id,
            section_id,
            label: Some("A-1".into()),
            is_enable: true,
            status,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mock_event_object_position(event_object_id: Uuid) -> event_object_position::Model {
        let now = mock_datetime();
        event_object_position::Model {
            id: Uuid::new_v4(),
            event_object_id,
            position_x: 0.0, // Assuming f64/f32. If i32, remove the .0
            position_y: 0.0,
            position_z: 0.0,
            rotation: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
}
