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
        let username: String = text_value_state_confirm.deref().into();
        onclick.emit(username);
    });

    html! {
        <div class="card-base">
        // <form>  // form tag does not work
          <div class="mb-6">
            <label for="username" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Username"}</label>
            <input type="text" onchange={onchange} id="username" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="username" required=true />
          </div>
          <div class="mb-6">
            <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Your password"}</label>
            <input type="password" id="password" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required=true />
          </div>
          <button type="submit" onclick={submit} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{"Submit"}</button>
        // <form>
        </div>
    }
}
