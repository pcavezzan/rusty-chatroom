use web_sys::{Event, HtmlTextAreaElement, MouseEvent};
use yew::{function_component, html, use_state, Callback, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct SendDialogProps {
    pub send_message_callback: Callback<String>,
    pub username: String,
}

#[function_component(SendDialog)]
pub fn send_dialog(props: &SendDialogProps) -> Html {
    let new_message_handle = use_state(String::new);
    let new_message = (*new_message_handle).clone();

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(textarea) = target {
            cloned_new_message_handle.set(textarea.value());
        }
    });

    let cloned_new_message = new_message.clone();
    let callback = props.send_message_callback.clone();
    let on_button_click = Callback::from(move |_: MouseEvent| {
        callback.emit(cloned_new_message.clone());
        new_message_handle.set("".to_string());
    });
    let username = props.username.clone();

    html! {
        <div class="input-group">
            <button class="btn btn-secondary">{ username }</button>
            <span class="input-group-text">{ "Your message:" }</span>
            <textarea class="form-control" onchange={on_message_change} value={new_message}></textarea>
            <button type="submit" onclick={on_button_click} class="btn-primary">{"Submit"}</button>
        </div>
    }
}
