use std::ops::Deref;
use web_sys::console::log_1;
use yew::Callback;

use yew::prelude::*;

use crate::components::atomics::text_input::{TextInput, INPUTTYPE};
use crate::services::backend::{authenticate, register_user};
use reqwasm::http::Request;
use serde::Serialize;

#[derive(Serialize, Debug, Default, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[function_component]
pub fn Register() -> Html {
    let state = use_state(RegisterRequest::default);

    let username_on_change = {
        let state = state.clone();
        Callback::from(move |value: String| {
            let mut state_handle = state.deref().clone();
            state_handle.username = value;
            state.set(state_handle.into());
        })
    };

    let password_on_change = {
        let state = state.clone();
        Callback::from(move |value: String| {
            let mut state_handle = state.deref().clone();
            state_handle.password = value;
            state.set(state_handle.clone());
        })
    };

    let submit = Callback::from(move |_| {
        let x = state.clone();
        let msg = format!("send register {:?}", x);
        log_1(&msg.into());
        register_user(x.deref());
    });

    html! {
        <form>
            <div class="mb-6">
                <TextInput id={"username"} input_type={Some(INPUTTYPE::Email)} on_change={username_on_change}/>
            </div>
            <div class="mb-6">
                <TextInput id={"password"} input_type={Some(INPUTTYPE::Password)} on_change={password_on_change}/>
            </div>
            <div class="flex items-center h-5">
                <input id="remember" type="checkbox" value="" class="w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800" required=true />
                <label for="remember" class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">{"I agree with the"} <a href="#" class="text-blue-600 hover:underline dark:text-blue-500">{" terms and conditions"}</a>{"."}</label>
            </div>
            <button onclick={submit} type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{"Create Account"}</button>
        </form>
    }
}
