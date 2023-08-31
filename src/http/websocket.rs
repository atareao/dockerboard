
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tracing::{info, debug, error};
use crate::http::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/websocket",
            routing::get(websocket_handler)
        )
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        debug!("{:?}", &message);
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            // If not empty we want to quit the loop else we want to quit function.
            if !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken.
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    //let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{} joined.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    //let mut send_task = tokio::spawn(async move {
    //    while let Ok(msg) = rx.recv().await {
    //        // In any websocket error, break loop.
    //        if sender.send(Message::Text(msg)).await.is_err() {
    //            break;
    //        }
    //    }
    //});

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("{}: {}", name, text));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    //tokio::select! {
    //    _ = (&mut send_task) => recv_task.abort(),
    //    _ = (&mut recv_task) => send_task.abort(),
    //};

    // Send "user left" message (similar to "joined" above).
    let msg = format!("{} left.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);

    // Remove username from map so new clients can take it again.
    //state.user_set.lock().unwrap().remove(&username);
}
