use common::ChatMessage;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<ChatMessage>,
}

#[function_component(MessageList)]
pub fn message_list(props: &MessageListProps) -> Html {
    html! {
        <div class="list-group">
        {
            props.messages.iter().map(|m| html! {
            <div class="list-group-item list-group-item-action">
                <div class="d-flex w-100 justify-content-between">
                    <h5>{m.author.clone()}</h5>
                    <small>{m.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</small>
                </div>
                <p>{m.message.clone()}</p>
            </div>
            }).collect::<Html>()
        }
        </div>
    }
}