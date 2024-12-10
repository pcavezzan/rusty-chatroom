use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::new);
    let messages = (*messages_handle).clone();
    let new_message_handle = use_state(String::new);
    let new_message = (*new_message_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();
    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            cloned_messages.push(ws_msg.clone());
            messages_handle.set(cloned_messages);
        }
    });

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(textarea) = target {
            let text_value = textarea.value();
            cloned_new_message_handle.set(text_value);
        }
    });

    let cloned_new_message = new_message.clone();
    let cloned_ws = ws.clone();
    let on_button_click = Callback::from(move |_: MouseEvent| {
        cloned_ws.send(cloned_new_message.clone());
        new_message_handle.set("".to_string());
    });

    html! {
        <>
            <ul id="chat">
            {
                messages.iter().map(|msg| html! {<li>{msg}</li>}).collect::<Html>()
            }
            </ul>
            <textarea onchange={on_message_change} value={new_message} ></textarea>
            <button type="submit" onclick={on_button_click}>{"Submit"}</button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
