use std::ops::Deref;

use yew::Callback;

use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_button_clicked: Callback<String>,
}

#[function_component]
pub fn Login(props: &Props) -> Html {
    let text_value = use_state(|| "none".to_string());
    let text_value_state = text_value.clone();

    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        text_value_state.set(input.value().into());
    });

    let text_value_state_confirm = text_value.clone();
    let onclick = props.on_button_clicked.clone();
    let submit = Callback::from(move |_| {
        let greeting = String::from(format!("callback called"));
        let username: String = text_value_state_confirm.deref().into();
        web_sys::console::log_1(&greeting.into());
        onclick.emit(username);
    });

    html! {
        // No properties
        <div class="card">
            <div> {"username"}
                <input type="text" value={"testuser"} onchange={onchange}/>
            </div>
            <button onclick={submit}>
                { "login" }
            </button>
        </div>

    }
}
