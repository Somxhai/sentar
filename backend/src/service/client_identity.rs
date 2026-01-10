use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    dto::cache::SessionCache,
    error::AppError,
    model::{event, workspace_member},
    service::workspace_role::WorkspaceRole,
};

#[derive(Clone, Debug)]
pub enum ClientIdentity {
    Authenticated(SessionCache),
    Guest,
}

impl ClientIdentity {
    pub fn get_session(self) -> Option<SessionCache> {
        match self {
            ClientIdentity::Authenticated(session) => Some(session),
            _ => None,
        }
    }

    pub async fn is_event_admin(
        &self,
        db: &DatabaseConnection,
        event_id: Uuid,
    ) -> eyre::Result<bool> {
        let session = match self {
            ClientIdentity::Authenticated(s) => s,
            ClientIdentity::Guest => return Ok(false),
        };

        let event_wrapper = event::Entity::find_by_id(event_id).one(db).await?;

        let event = match event_wrapper {
            Some(e) => e,
            None => return Err(AppError::NotFound("Event not found".into()).into()),
        };

        let membership_count = workspace_member::Entity::find()
            .filter(workspace_member::Column::WorkspaceId.eq(event.workspace_id))
            .filter(workspace_member::Column::UserId.eq(&session.user_id))
            .filter(workspace_member::Column::Role.eq("admin"))
            .count(db)
            .await?;

        Ok(membership_count > 0)
    }

    pub async fn get_workspace_role(
        &self,
        db: &DatabaseConnection,
        workspace_id: Uuid,
    ) -> eyre::Result<WorkspaceRole> {
        let session = match self {
            ClientIdentity::Authenticated(s) => s,
            ClientIdentity::Guest => return Ok(WorkspaceRole::Guest),
        };

        let workspace = workspace_member::Entity::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::UserId.eq(&session.user_id))
            .one(db)
            .await?;

        match workspace {
            Some(workspace_member) => Ok(WorkspaceRole::from(workspace_member.role)),
            None => Ok(WorkspaceRole::Guest),
        }
    }
}
