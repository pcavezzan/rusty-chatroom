mod message_list;
mod send_dialog;
mod users_list;

use crate::message_list::MessageList;
use crate::send_dialog::SendDialog;
use common::{WebSocketMessage, WebSocketMessageType};
use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::new);
    let messages = (*messages_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();
    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            let websocket_message: WebSocketMessage = serde_json::from_str(&ws_msg).unwrap();
            match websocket_message.message_type {
                WebSocketMessageType::NewMessage => {
                    if let Some(msg) = websocket_message.message {
                        cloned_messages.push(msg);
                        messages_handle.set(cloned_messages);
                    }
                }
                WebSocketMessageType::UsersList => {}
            }
        }
    });

    let cloned_ws = ws.clone();
    let send_message_callback = Callback::from(move |msg: String| {
        cloned_ws.send(msg.clone());
    });

    html! {
        <div class="container">
            <div class="row">
                <MessageList messages={messages} />
            </div>
            <div class="row">
                <SendDialog send_message_callback={send_message_callback} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
