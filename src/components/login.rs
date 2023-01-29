use std::ops::Deref;
use crate::components::atomics::text_input::TextInput;
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
    let text_value = use_state(|| "".to_string());
    let text_value_state = text_value.clone();

    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        text_value_state.set(input.value().into());
    });

    let text_value_state_confirm = text_value.clone();
    let onclick = props.on_button_clicked.clone();
    let submit = Callback::from(move |_| {
        let username: String = text_value_state_confirm.deref().into();
        onclick.emit(username);
    });

    html! {
        <div class="card-base">
        // <form>  // form tag does not work
          <div class="mb-6">
            <label for="username" class="input-label">{"Username"}</label>
            <input type="text" onchange={onchange} id="username" class="input" placeholder="username" required=true />
          </div>
          <div class="mb-6">
            <label for="password" class="input-label">{"Your password"}</label>
            <input type="password" id="password" class="input" required=true />
          </div>
          <button type="submit" onclick={submit} class="button">{"Submit"}</button>
        // <form>

        <div class="mb-6">
            <TextInput id={"password"} />
        </div>
        </div>
    }
}
