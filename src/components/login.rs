use crate::components::atomics::text_input::TextInput;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::console::log_1;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Callback;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_button_clicked: Callback<LoginRequest>,
}

#[derive(Serialize, Debug, Default, Clone)]
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
    let state = use_state(LoginRequest::default);

    let username_on_change = {
        let state = state.clone();
        Callback::from(move |value: String| {
            let mut login_request = state.deref().clone();
            login_request.username = value;
            state.set(login_request);
        })
    };

    let password_on_change = {
        let state = state.clone();
        Callback::from(move |value: String| {
            let mut login_request = state.deref().clone();
            login_request.password = value;
            state.set(login_request);
        })
    };

    let onclick = props.on_button_clicked.clone();
    let submit = Callback::from(move |_| {
        onclick.emit(state.deref().clone());
    });

    html! {
        <div class="card-base">
        <div class="mb-6">
            <TextInput id={"username"} on_change={username_on_change}/>
            <TextInput id={"password"} on_change={password_on_change}/>
        </div>
        <button type="submit" onclick={submit} class="button">{"Submit"}</button>

        </div>
    }
}
