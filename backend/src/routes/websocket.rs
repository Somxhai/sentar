use crate::{
    error::AppError,
    model::event,
    service::{
        client_identity::ClientIdentity, workspace_role::WorkspaceRole, ws_message::WSMessage,
    },
};
use axum::{
    extract::{Path, Query, State, WebSocketUpgrade, ws::Message},
    response::Response,
};
use sea_orm::{EntityTrait, QuerySelect};
use tokio::sync::broadcast;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    app::{
        AppState,
        web_socket::{ActionResult, BroadcastMessage},
    },
    routes::websocket::orchestrator::handle_command,
    utils::auth::validate_token,
};
use axum::extract::ws::WebSocket;
use futures_util::{SinkExt, StreamExt};

mod orchestrator;

#[derive(serde::Deserialize)]
pub struct WsParams {
    pub token: Option<String>,
}

pub async fn websocket_handler(
    Path(event_id): Path<Uuid>,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<WsParams>,
) -> Result<Response, AppError> {
    let identity = if let Some(token) = params.token {
        match validate_token(&token, &state.db, &state.cache).await {
            Ok(session) => ClientIdentity::Authenticated(session),
            Err(_) => {
                return Err(AppError::Unauthorized);
            }
        }
    } else {
        ClientIdentity::Guest
    };

    let event = match event::Entity::find_by_id(event_id)
        .select_only()
        .column(event::Column::WorkspaceId)
        .one(&*state.db)
        .await
    {
        Ok(Some(e)) => e,
        Ok(None) => {
            return Err(AppError::NotFound("Event not found".into()));
        }
        Err(_) => {
            return Err(AppError::Internal);
        }
    };

    let workspace_id = event.workspace_id;

    let role = if let Ok(role) = identity.get_workspace_role(&state.db, workspace_id).await {
        role
    } else {
        crate::service::workspace_role::WorkspaceRole::Guest
    };

    Ok(ws.on_upgrade(move |socket| socket_handler(socket, state, event_id, identity, role)))
}

pub async fn socket_handler(
    socket: WebSocket,
    state: AppState,
    event_id: Uuid,
    identity: ClientIdentity,
    workspace_role: WorkspaceRole,
) {
    // split the websocket stream into a sender (sink) and receiver (stream)
    let (mut sink, mut stream) = socket.split();
    // create an mpsc so we can send messages to the sink from multiple threads
    let (sender, receiver) = flume::unbounded::<BroadcastMessage>();

    let room_tx = state
        .rooms
        .entry(event_id)
        .or_insert_with(|| {
            let (tx, _rx) = broadcast::channel(1024);
            tx
        })
        .clone();

    // spawn a task that forwards messages from the mpsc to the sink
    // send data to user
    let mut write_to_mpsc_task = tokio::spawn(async move {
        while let Ok(message) = receiver.recv_async().await {
            match serde_json::to_string(&message) {
                Ok(msg) => {
                    if sink.send(msg.into()).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to parse JSON: {}", e);
                }
            }
        }
    });

    let mut rx_chat = room_tx.subscribe();

    // whenever a chat is sent to rx_chat, forward it to the mpsc
    let bridge_sender = sender.clone();
    let mut bridge_task = tokio::spawn(async move {
        loop {
            match rx_chat.recv().await {
                Ok(msg) => {
                    if bridge_sender
                        .send_async(BroadcastMessage::Event { event: msg })
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    warn!("Client lagged: skipped {} messages", n);
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    let tx_for_room = room_tx.clone();
    let identity_for_recv = identity.clone();
    // recieve data from user
    let ack_sender = sender.clone();
    let mut read_task = tokio::spawn(async move {
        while let Some(result) = stream.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    warn!("WebSocket stream error: {}", e);
                    break;
                }
            };

            match msg {
                Message::Text(text) => {
                    let cmd = match serde_json::from_str::<WSMessage>(&text) {
                        Ok(v) => v,
                        Err(e) => {
                            let req_id = serde_json::from_str::<serde_json::Value>(&text)
                                .ok()
                                .and_then(|v| {
                                    v.get("request_id")
                                        .and_then(|id| id.as_str())
                                        .map(|s| s.to_string())
                                })
                                .unwrap_or_else(|| "unknown".to_string());

                            error!(%req_id, "Invalid WS JSON: {}", e);

                            let _ = ack_sender
                                .send_async(BroadcastMessage::Error {
                                    request_id: req_id,
                                    code: "400".into(),
                                    message: format!("Invalid JSON: {}", e),
                                })
                                .await;

                            continue;
                        }
                    };

                    match handle_command(
                        &state.db,
                        event_id,
                        cmd,
                        &identity_for_recv,
                        workspace_role,
                    )
                    .await
                    {
                        ActionResult::Ack { ack } => {
                            let _ = ack_sender.send_async(ack).await;
                        }
                        ActionResult::AckAndBroadcast { ack, event } => {
                            let _ = ack_sender.send_async(ack).await;
                            let _ = tx_for_room.send(event);
                        }
                        ActionResult::Error {
                            request_id,
                            code,
                            message,
                        } => {
                            let _ = ack_sender
                                .send_async(BroadcastMessage::Error {
                                    request_id,
                                    code,
                                    message,
                                })
                                .await;
                        }
                    }
                }
                Message::Ping(_) => {}
                Message::Pong(_) => {}
                Message::Binary(_) => {
                    warn!("Unexpected binary message received");
                }
                Message::Close(_) => {
                    info!("Client disconnected");
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut bridge_task) => { read_task.abort(); write_to_mpsc_task.abort(); },
        _ = (&mut read_task) => { bridge_task.abort(); write_to_mpsc_task.abort(); },
        _ = (&mut write_to_mpsc_task) => { read_task.abort(); bridge_task.abort(); },
    };
}
