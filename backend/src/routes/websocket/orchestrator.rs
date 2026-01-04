use axum::http::StatusCode;
use sea_orm::DatabaseConnection;
use tracing::error;
use uuid::Uuid;

use crate::service::client_identity::ClientIdentity;
use crate::service::layout::LayoutManagement;
use crate::service::reservation::ReservationBuilder;
use crate::service::user_seat_management::UserSeatManagement;
use crate::service::workspace_role::WorkspaceRole;
use crate::service::ws_message::WSMessage;
use crate::{
    app::web_socket::{BroadcastMessage, ServerEvent},
    routes::websocket::ActionResult,
};

pub async fn handle_command(
    db: &DatabaseConnection,
    event_id: Uuid,
    cmd: WSMessage,
    identity: &ClientIdentity,
    workspace_role: WorkspaceRole,
) -> ActionResult {
    let identity = identity.clone();
    match cmd {
        WSMessage::Reserve {
            request_id,
            seat_ids,
        } => {
            let Some(session) = identity.get_session() else {
                return ActionResult::Error {
                    request_id,
                    code: "401".into(),
                    message: "Must be logged in user.".into(),
                };
            };

            let by = session.user_id;

            let reservation = ReservationBuilder::new(db, event_id, by.clone());

            if (reservation.seats(seat_ids.clone()).reserve().await).is_err() {
                return ActionResult::Error {
                    request_id,
                    code: "500".into(),
                    message: "Something went wrong".into(),
                };
            }

            ActionResult::AckAndBroadcast {
                ack: BroadcastMessage::Ack { request_id },
                event: ServerEvent::SeatReserved { seat_ids, by },
            }
        }

        WSMessage::Release {
            request_id,
            seat_id,
        } => {
            let Some(session) = identity.get_session() else {
                return ActionResult::Error {
                    request_id,
                    code: "401".into(),
                    message: "Must be logged in user.".into(),
                };
            };

            let seat_manager = UserSeatManagement::new(db, session.user_id);

            if let Err(e) = seat_manager.release(seat_id).await {
                return ActionResult::Error {
                    request_id,
                    code: "500".into(),
                    message: e.to_string(),
                };
            }

            ActionResult::AckAndBroadcast {
                ack: BroadcastMessage::Ack { request_id },
                event: ServerEvent::SeatReleased { seat_id },
            }
        }

        WSMessage::MoveObjectInLayout {
            request_id,
            seat_id,
            x,
            y,
            z,
        } => {
            let Some(_session) = identity.get_session() else {
                return ActionResult::Error {
                    request_id,
                    code: "401".into(),
                    message: "Must be logged in user.".into(),
                };
            };

            if workspace_role != WorkspaceRole::Admin {
                return ActionResult::Error {
                    request_id,
                    code: "403".into(),
                    message: "You do not have permission.".into(),
                };
            }

            let layout_management = LayoutManagement::new(db, event_id);

            match layout_management.move_object(seat_id, x, y, z).await {
                Ok(updated_position) => ActionResult::AckAndBroadcast {
                    ack: BroadcastMessage::Ack { request_id },
                    event: ServerEvent::SeatMoved {
                        seat_id,
                        x: updated_position.position_x,
                        y: updated_position.position_y,
                        z: updated_position.position_z,
                    },
                },
                Err(e) => {
                    error!("Failed to move object: {:?}", e);
                    ActionResult::Error {
                        request_id,
                        code: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                        message: "Failed to update object position.".into(),
                    }
                }
            }
        }
    }
}
