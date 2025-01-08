use web_sys::{Event, HtmlInputElement, HtmlTextAreaElement, MouseEvent};
use yew::{function_component, html, use_state, Callback, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct SendDialogProps {
    pub send_message_callback: Callback<String>,
    pub change_username_callback: Callback<String>,
    pub username: String,
}

#[function_component(SendDialog)]
pub fn send_dialog(props: &SendDialogProps) -> Html {
    let new_message_handle = use_state(String::new);
    let new_message = (*new_message_handle).clone();
    let is_editing_username_handle = use_state(|| false);
    let is_editing_username = (*is_editing_username_handle).clone();
    let new_username_handle = use_state(|| props.username.clone());
    let new_username = (*new_username_handle).clone();

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
    let cloned_is_editing_username_handle = is_editing_username_handle.clone();
    let on_username_change_click = Callback::from(move |_| {
        cloned_is_editing_username_handle.set(true);
    });
    let cloned_is_editing_username_handle = is_editing_username_handle.clone();
    let on_username_edit_cancel_click = Callback::from(move |_| {
        cloned_is_editing_username_handle.set(false);
    });
    let callback = props.change_username_callback.clone();
    let cloned_new_username = new_username.clone();
    let on_username_edit_apply_click = Callback::from(move |_| {
        callback.emit(cloned_new_username.clone());
        is_editing_username_handle.set(false);
    });
    let on_username_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            new_username_handle.set(input.value());
        }
    });

    html! {
        <div class="input-group">
            if is_editing_username {
                <input type="text" class="form-control" value={ new_username }  onchange={ on_username_change } />
                <button class="btn btn-secondary" onclick={ on_username_edit_apply_click }>{ "v" }</button>
                <button class="btn btn-danger" onclick={ on_username_edit_cancel_click }>{ "x" }</button>
            } else {
                <button class="btn btn-secondary" onclick={ on_username_change_click }>{ username }</button>
            }
            <span class="input-group-text">{ "Your message:" }</span>
            <textarea class="form-control" onchange={on_message_change} value={new_message}></textarea>
            <button type="submit" onclick={on_button_click} class="btn-primary">{"Submit"}</button>
        </div>
    }
}
