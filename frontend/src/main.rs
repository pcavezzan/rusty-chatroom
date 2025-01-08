mod message_list;
mod send_dialog;
mod users_list;

use crate::message_list::MessageList;
use crate::send_dialog::SendDialog;
use crate::users_list::UsersList;
use common::{ChatMessage, WebSocketMessage, WebSocketMessageType};
use serde_json::json;
use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::new);
    let messages = (*messages_handle).clone();
    let users_handle = use_state(Vec::new);
    let users = (*users_handle).clone();
    let username_handler = use_state(String::default);
    let username = (*username_handler).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();
    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            let websocket_message: WebSocketMessage = serde_json::from_str(&ws_msg).unwrap();
            match websocket_message.message_type {
                WebSocketMessageType::NewMessage => {
                    let msg = websocket_message.message.expect("Missing message payload");
                    cloned_messages.push(msg);
                    messages_handle.set(cloned_messages);
                }
                WebSocketMessageType::UsersList => {
                    let users = websocket_message.users.expect("Missing users payload");
                    users_handle.set(users);
                }
                WebSocketMessageType::UsernameChange => {
                    let username = websocket_message
                        .username
                        .expect("Missing username payload");
                    username_handler.set(username);
                }
            }
        }
    });

    let cloned_ws = ws.clone();
    let author = username.clone();
    let send_message_callback = Callback::from(move |msg: String| {
        let websocket_message = WebSocketMessage {
            message_type: WebSocketMessageType::NewMessage,
            message: Some(ChatMessage {
                message: msg,
                author: author.clone(),
                created_at: chrono::Utc::now().naive_utc(),
            }),
            users: None,
            username: None,
        };
        cloned_ws.send(json!(websocket_message).to_string());
    });

    let cloned_ws = ws.clone();
    let change_username_callback = Callback::from(move |username: String| {
        let websocket_message = WebSocketMessage {
            message_type: WebSocketMessageType::UsernameChange,
            message: None,
            users: None,
            username: Some(username),
        };
        cloned_ws.send(json!(websocket_message).to_string());
    });

    html! {
        <div class="container-fluid">
            <div class="row">
                <div class="col-sm-3">
                    <UsersList users={users} />
                </div>
                <div class="col-sm-9">
                    <MessageList messages={messages} />
                </div>
            </div>
            <div class="row">
                if !username.is_empty() {
                    <SendDialog send_message_callback={send_message_callback} change_username_callback={change_username_callback} username={username} />
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
