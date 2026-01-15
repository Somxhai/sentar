use backend::model::{
    event, event_object, event_object_position, form, reservation, section, user, workspace,
    workspace_member,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use uuid::Uuid;

#[allow(dead_code)]
pub struct Seeding<'a>(pub &'a DatabaseConnection);

#[allow(dead_code)]
impl<'a> Seeding<'a> {
    pub async fn create_user(&self, id: String) -> user::Model {
        let active_model = user::ActiveModel {
            id: Set(id),
            name: Set(format!("user_{}", Uuid::new_v4())),
            email: Set("test@email".into()),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create user")
    }

    pub async fn create_workspace(&self, name: &str, owner_id: &str) -> workspace::Model {
        let active_model = workspace::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name.to_string()),
            owner_id: Set(owner_id.to_string()),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create workspace")
    }

    pub async fn create_section(
        &self,
        id: Uuid,
        title: &str,
        event_id: Uuid,
        price: f64,
    ) -> section::Model {
        let active_model = section::ActiveModel {
            id: Set(id),
            event_id: Set(event_id),
            title: Set(title.to_string()),
            price: Set(price),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create section")
    }

    pub async fn create_event(&self, title: &str, workspace_id: Uuid) -> event::Model {
        let active_model = event::ActiveModel {
            id: Set(Uuid::new_v4()),
            workspace_id: Set(workspace_id),
            title: Set(title.to_string()),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create event")
    }

    pub async fn create_form(
        &self,
        id: Uuid,
        event_id: Uuid,
        title: &str,
        description: &str,
        user_id: &str,
    ) -> form::Model {
        let active_model = form::ActiveModel {
            id: Set(id),
            event_id: Set(event_id),
            title: Set(Some(title.to_string())),
            description: Set(Some(description.to_string())),
            updated_by: Set(user_id.to_string()),
            is_active: Set(false),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create form")
    }

    pub async fn create_reservation(&self, user_id: String, event_id: Uuid) -> reservation::Model {
        let active_model = reservation::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            event_id: Set(event_id),
            status: Set("on_hold".to_string()),
            total_price: Set(0.0),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create reservation")
    }

    pub async fn create_workspace_member(
        &self,
        workspace_id: Uuid,
        user_id: String,
        invited_by: String,
    ) -> workspace_member::Model {
        let active_model = workspace_member::ActiveModel {
            id: Set(Uuid::new_v4()),
            workspace_id: Set(workspace_id),
            user_id: Set(user_id),
            invited_by: Set(invited_by),
            status: Set("pending".into()),
            role: Set("admin".into()),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create workspace member")
    }

    pub async fn create_event_object(
        &self,
        event_id: Uuid,
        section_id: Option<Uuid>,
        status: String,
    ) -> event_object::Model {
        let id = Uuid::new_v4();
        let active_model = event_object::ActiveModel {
            id: Set(id),
            object_type: Set("seat".into()),
            event_id: Set(event_id),
            section_id: Set(section_id),
            label: Set(Some("A-1".into())),
            is_enable: Set(true),
            status: Set(status),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create event object")
    }

    pub async fn create_event_object_position(
        &self,
        event_object_id: Uuid,
    ) -> event_object_position::Model {
        let active_model = event_object_position::ActiveModel {
            id: Set(Uuid::new_v4()),
            event_object_id: Set(event_object_id),
            position_x: Set(0.0),
            position_y: Set(0.0),
            position_z: Set(0.0),
            rotation: Set(0.0),
            ..Default::default()
        };

        active_model
            .insert(self.0)
            .await
            .expect("Can not create event object position")
    }
}
