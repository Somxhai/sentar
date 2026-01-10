use sea_orm::{MockDatabase, MockExecResult};
use uuid::Uuid;

use crate::common::helpers::{
    mock_event, mock_event_object, mock_event_object_position, mock_workspace_member,
};

#[allow(dead_code)]
pub struct LayoutTest {
    pub event_id: Uuid,
    pub user_id: String,
    pub seat_id: Uuid,
    pub workspace_id: Uuid,
}

impl LayoutTest {
    pub fn new(event_id: Uuid, user_id: String, seat_id: Uuid, workspace_id: Uuid) -> Self {
        Self {
            event_id,
            user_id,
            seat_id,
            workspace_id,
        }
    }

    pub fn prepare_layout(&self) -> MockDatabase {
        let event_owner_id = "iam_owner_id".to_string();

        // 1. Handshake Data
        let event = [mock_event(self.event_id, "event_title", self.workspace_id)];
        let member = [mock_workspace_member(
            self.workspace_id,
            self.user_id.clone(),
            event_owner_id,
        )];

        let mut seat = mock_event_object(self.event_id, None, "waiting".into());
        seat.id = self.seat_id;
        seat.status = "available".into();
        let available_seats = [seat];

        let position = [mock_event_object_position(self.seat_id)];

        MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([event]) // 1. Validate Event
            .append_query_results([member]) // 2. Validate Member
            .append_query_results(vec![join_result])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
    }
}
