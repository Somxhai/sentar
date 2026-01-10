use std::collections::BTreeMap;

use backend::model::reservation_item;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Value};
use uuid::Uuid;

use crate::common::helpers::{
    mock_datetime, mock_event, mock_event_object, mock_reservation, mock_workspace_member,
};

pub struct ReservationTest {
    pub event_id: Uuid,
    pub user_id: String,
    pub seat_id: Uuid,
    pub workspace_id: Uuid,
}
#[allow(dead_code)]
impl ReservationTest {
    pub fn new(event_id: Uuid, user_id: String, seat_id: Uuid, workspace_id: Uuid) -> Self {
        Self {
            event_id,
            user_id,
            seat_id,
            workspace_id,
        }
    }

    pub fn prepare_reservation(&self) -> MockDatabase {
        let event_owner_id = "iam_owner_id".to_string();

        // 1. Handshake Data
        let event = [mock_event(self.event_id, "event_title", self.workspace_id)];
        let member = [mock_workspace_member(
            self.workspace_id,
            self.user_id.clone(),
            event_owner_id,
        )];

        // 2. Handler Data: Available Seat
        let mut seat = mock_event_object(self.event_id, None, "waiting".into());
        seat.id = self.seat_id;
        seat.status = "available".into();
        let available_seats = [seat];

        // 3. Handler Data: Created Reservation
        let res_model = [mock_reservation(self.user_id.clone(), self.event_id)];

        // 4. Handler Data: Created Reservation Item
        let now = mock_datetime();
        let res_item = [reservation_item::Model {
            id: Uuid::new_v4(),
            reservation_id: res_model[0].id,
            event_object_id: self.seat_id,
            price_at_booking: 0.0,
            created_at: now,
            updated_at: now,
        }];

        MockDatabase::new(DatabaseBackend::Postgres)
            // --- Handshake Phase ---
            .append_query_results([event]) // 1. Validate Event
            .append_query_results([member]) // 2. Validate Member
            // --- Reserve Handler Phase ---
            .append_query_results([available_seats]) // 3. Find Seat
            .append_query_results([res_model]) // 4. Insert Res
            .append_query_results([res_item]) // 5. Insert Item
            .append_exec_results([MockExecResult {
                // 6. Update Seat
                last_insert_id: 0,
                rows_affected: 1,
            }])
    }

    pub fn prepare_release(&self) -> MockDatabase {
        let event_owner_id = "iam_owner_id".to_string();

        // --- Handshake Data ---
        let event = vec![mock_event(self.event_id, "event_title", self.workspace_id)];
        let member = vec![mock_workspace_member(
            self.workspace_id,
            self.user_id.clone(),
            event_owner_id,
        )];

        // --- Handler Data ---

        // 1. Prepare data for "Find Item + Reservation"
        let res_model = mock_reservation(self.user_id.clone(), self.event_id);
        let now = mock_datetime();
        let item_model = reservation_item::Model {
            id: Uuid::new_v4(),
            reservation_id: res_model.id,
            event_object_id: self.seat_id,
            price_at_booking: 0.0,
            created_at: now,
            updated_at: now,
        };
        // The result of find_also_related is a tuple
        let join_result = vec![(item_model, Some(res_model.clone()))];

        // 2. Prepare data for "Count"
        // SeaORM count queries expect a column named "num_items"
        let mut count_row = BTreeMap::new();
        count_row.insert("num_items".to_string(), Value::BigInt(Some(0)));

        // 3. Prepare data for "Update Reservation" (Cancelled)
        let mut res_cancelled = res_model.clone();
        res_cancelled.status = "cancelled".into();

        MockDatabase::new(DatabaseBackend::Postgres)
            // 1. Handshake: Event Check
            .append_query_results(vec![event])
            // 2. Handshake: Member Check
            .append_query_results(vec![member])
            // 3. Handler: Find Item + Reservation (Query)
            .append_query_results(vec![join_result])
            // 4. Handler: Delete Item (Exec)
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            // 5. Handler: Update Seat Status (Exec)
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            // 6. Handler: Count Remaining Items (Query)
            .append_query_results(vec![vec![count_row]])
            // 7. Handler: Cancel Reservation (Query - Update Returning)
            .append_query_results(vec![vec![res_cancelled]])
    }
}
