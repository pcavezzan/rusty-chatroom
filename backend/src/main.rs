use common::{ChatMessage, WebSocketMessage, WebSocketMessageType};
use rocket::futures::stream::SplitSink;
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio::sync::Mutex;
use rocket::{routes, State};
use rocket_ws::stream::DuplexStream;
use rocket_ws::{Channel, Message, WebSocket};
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
struct ChatRoom {
    connections: Mutex<HashMap<usize, ChatRoomConnection>>,
}

struct ChatRoomConnection {
    username: String,
    sink: SplitSink<DuplexStream, Message>,
}

impl ChatRoom {
    pub async fn add(&self, id: usize, sink: SplitSink<DuplexStream, Message>) {
        let mut cons = self.connections.lock().await;
        let connection = ChatRoomConnection {
            username: format!("User #{}", id),
            sink,
        };
        cons.insert(id, connection);
    }

    pub async fn remove(&self, id: usize) {
        let mut cons = self.connections.lock().await;
        cons.remove(&id);
    }

    pub async fn broadcast_message(&self, message: ChatMessage) {
        let mut cons = self.connections.lock().await;
        let web_socket_message = WebSocketMessage {
            message_type: WebSocketMessageType::NewMessage,
            message: Some(message),
            users: None,
            username: None,
        };
        for (_, conn) in cons.iter_mut() {
            let _ = conn
                .sink
                .send(Message::Text(json!(web_socket_message).to_string()))
                .await;
        }
    }

    pub async fn broadcast_user_list(&self) {
        let mut cons = self.connections.lock().await;
        let mut users = vec![];
        for (_, conn) in cons.iter_mut() {
            users.push(conn.username.clone());
        }
        let web_socket_message = WebSocketMessage {
            message_type: WebSocketMessageType::UsersList,
            message: None,
            users: Some(users),
            username: None,
        };
        for (_, conn) in cons.iter_mut() {
            let _ = conn
                .sink
                .send(Message::Text(json!(web_socket_message).to_string()))
                .await;
        }
    }

    pub async fn send_username(&self, id: usize) {
        let mut conns = self.connections.lock().await;
        if let Some(conn) = conns.get_mut(&id) {
            let web_socket_message = WebSocketMessage {
                message_type: WebSocketMessageType::UsernameChange,
                message: None,
                users: None,
                username: Some(conn.username.clone()),
            };
            let _ = conn
                .sink
                .send(Message::Text(json!(web_socket_message).to_string()))
                .await;
        }
    }

    pub async fn change_username(&self, new_username: String, id: usize) {
        let mut conns = self.connections.lock().await;
        if let Some(conn) = conns.get_mut(&id) {
            conn.username = new_username;
        }
    }
}

#[rocket::get("/")]
fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();

            state.add(user_id, ws_sink).await;
            state.broadcast_user_list().await;
            state.send_username(user_id).await;

            while let Some(message_result) = ws_stream.next().await {
                if let Ok(message) = message_result {
                    handle_incoming_message(message, state, user_id).await;
                }
            }

            state.remove(user_id).await;
            state.broadcast_user_list().await;

            Ok(())
        })
    })
}

async fn handle_incoming_message(
    message_content: Message,
    state: &State<ChatRoom>,
    conn_id: usize,
) {
    match message_content {
        Message::Text(text) => {
            if let Ok(websocket_message) = serde_json::from_str::<WebSocketMessage>(&text) {
                match websocket_message.message_type {
                    WebSocketMessageType::NewMessage => {
                        if let Some(ws_msg) = websocket_message.message {
                            state.broadcast_message(ws_msg).await;
                        }
                    }
                    WebSocketMessageType::UsernameChange => {
                        if let Some(ws_username) = websocket_message.username {
                            state.change_username(ws_username, conn_id).await;
                            state.send_username(conn_id).await;
                            state.broadcast_user_list().await;
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {
            // Only text supported
        }
    }
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![chat])
        .manage(ChatRoom::default())
        .launch()
        .await;
}
