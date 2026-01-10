mod common;
use std::vec;

use backend::app::web_socket::BroadcastMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::{layout::LayoutTest, reservation::ReservationTest, server::create_test_app};

const TOKEN: &str = "fake-token-xxx";

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct ReserveJson {
    action: String,
    request_id: String,
    seat_ids: Vec<Uuid>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct ReleaseJson {
    action: String,
    request_id: String,
    seat_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct MoveObjectInLayout {
    action: String,
    request_id: String,

    seat_id: Uuid,
    x: f64,
    y: f64,
    z: f64,
}

#[tokio::test]
async fn test_reservation() -> eyre::Result<()> {
    let user_id = "my_user_id".to_string();
    let event_id = Uuid::new_v4();
    let seat1 = Uuid::new_v4();
    let workspace_id = Uuid::new_v4();

    let mock_db =
        ReservationTest::new(event_id, user_id, seat1, workspace_id).prepare_reservation();

    let server = create_test_app(mock_db).await?;

    let request = server.get_websocket(&format!("/ws/{}?token={}", event_id, TOKEN));

    let response = request.await;

    let mut socket = response.into_websocket().await;

    socket
        .send_json(&ReserveJson {
            action: "reserve".to_string(),
            request_id: Uuid::new_v4().to_string(),
            seat_ids: vec![seat1],
        })
        .await;

    let response: BroadcastMessage = socket.receive_json().await;
    assert!(matches!(response, BroadcastMessage::Ack { .. }));
    println!("output -> {:?}", response);

    Ok(())
}

#[tokio::test]
async fn test_release_object_event() -> eyre::Result<()> {
    let user_id = "my_user_id".to_string();
    let event_id = Uuid::new_v4();
    let seat_id = Uuid::new_v4();
    let workspace_id = Uuid::new_v4();
    let mock_db = ReservationTest::new(event_id, user_id, seat_id, workspace_id).prepare_release();
    let server = create_test_app(mock_db).await?;

    let request = server.get_websocket(&format!("/ws/{}?token={}", event_id, TOKEN));

    let response = request.await;

    let mut socket = response.into_websocket().await;

    socket
        .send_json(&ReleaseJson {
            action: "release".to_string(),
            request_id: Uuid::new_v4().to_string(),
            seat_id: seat_id,
        })
        .await;
    let response: BroadcastMessage = socket.receive_json().await;
    println!("output -> {:?}", response);
    assert!(matches!(response, BroadcastMessage::Ack { .. }));
    Ok(())
}

// #[tokio::test]
// async fn test_move_object_in_layout() -> eyre::Result<()> {
//     let user_id = "my_user_id".to_string();
//     let event_id = Uuid::new_v4();
//     let seat_id = Uuid::new_v4();
//     let workspace_id = Uuid::new_v4();
//
//     let layout_manager = LayoutTest::new(event_id, user_id, seat_id, workspace_id);
//     let mock_db = layout_manager.prepare_layout();
//
//     let server = create_test_app(mock_db).await?;
//
//     let request = server.get_websocket(&format!("/ws/{}?token={}", event_id, TOKEN));
//
//     let response = request.await;
//
//     let mut socket = response.into_websocket().await;
//
//     socket
//         .send_json(&MoveObjectInLayout {
//             action: "release".to_string(),
//             request_id: Uuid::new_v4().to_string(),
//             seat_id: seat_id,
//             x: 3.0,
//             y: 67.0,
//             z: -2.0,
//         })
//         .await;
//     let response: BroadcastMessage = socket.receive_json().await;
//     println!("output -> {:?}", response);
//     assert!(matches!(response, BroadcastMessage::Ack { .. }));
//     Ok(())
// }
