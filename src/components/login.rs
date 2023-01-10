use std::ops::Deref;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Callback;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_button_clicked: Callback<String>,
}

#[derive(Serialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
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
                <input type="text" onchange={onchange}/>
            </div>
            <button onclick={submit}>
                { "login" }
            </button>
        </div>

    }
}
