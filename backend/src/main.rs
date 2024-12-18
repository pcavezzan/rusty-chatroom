use chrono::Utc;
use common::{ChatMessage, WebSocketMessage, WebSocketMessageType};
use rocket::async_stream::stream;
use rocket::futures::stream::SplitSink;
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio::sync::Mutex;
use rocket::{routes, State};
use rocket_ws::stream::DuplexStream;
use rocket_ws::{Channel, Message, WebSocket};
use serde_json::json;
use std::collections::HashMap;
use std::os::macos::raw::stat;
use std::sync::atomic::{AtomicUsize, Ordering};

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
struct ChatRoom {
    connections: Mutex<HashMap<usize, SplitSink<DuplexStream, Message>>>,
}

impl ChatRoom {
    pub async fn add(&self, id: usize, sink: SplitSink<DuplexStream, Message>) {
        let mut cons = self.connections.lock().await;
        cons.insert(id, sink);
    }

    pub async fn remove(&self, id: usize) {
        let mut cons = self.connections.lock().await;
        cons.remove(&id);
    }

    pub async fn broadcast_message(&self, message: Message, user_id: usize) {
        let chat_message = ChatMessage {
            message: message.to_string(),
            author: format!("User #{}", user_id),
            created_at: Utc::now().naive_utc(),
        };
        let web_socket_message = WebSocketMessage {
            message_type: WebSocketMessageType::NewMessage,
            message: Some(chat_message),
            users: None,
        };
        let mut cons = self.connections.lock().await;
        for (_, sink) in cons.iter_mut() {
            let _ = sink
                .send(Message::Text(json!(web_socket_message).to_string()))
                .await;
        }
    }

    pub async fn broadcast_user_list(&self) {
        let mut cons = self.connections.lock().await;
        let mut users = vec![];
        for (id, _) in cons.iter_mut() {
            users.push(format!("User #{}", id));
        }
        let web_socket_message = WebSocketMessage {
            message_type: WebSocketMessageType::NewMessage,
            message: None,
            users: Some(users),
        };
        for (_, sink) in cons.iter_mut() {
            let _ = sink
                .send(Message::Text(json!(web_socket_message).to_string()))
                .await;
        }
    }
}

#[rocket::get("/")]
fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();

            state.add(user_id, ws_sink).await;
            state.broadcast_user_list().await;

            while let Some(message) = ws_stream.next().await {
                state.broadcast_message(message?, user_id).await;
            }

            state.remove(user_id).await;
            state.broadcast_user_list().await;

            Ok(())
        })
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![chat])
        .manage(ChatRoom::default())
        .launch()
        .await;
}
