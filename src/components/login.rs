use crate::components::atomics::text_input::{TextInput, INPUTTYPE};
use serde::Serialize;
use std::ops::Deref;
use yew::prelude::*;
use yew::Callback;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_button_clicked: Callback<LoginRequest>,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[function_component]
pub fn Login(props: &Props) -> Html {
    let state = use_state(LoginRequest::default);

    let email_on_change = {
        let state = state.clone();
        Callback::from(move |value: String| {
            let mut state_handle = state.deref().clone();
            state_handle.email = value;
            state.set(state_handle);
        })
    };

    let password_on_change = {
        let state = state.clone();
        Callback::from(move |value: String| {
            let mut state_handle = state.deref().clone();
            state_handle.password = value;
            state.set(state_handle);
        })
    };

    let onclick = props.on_button_clicked.clone();
    let submit = Callback::from(move |_| {
        onclick.emit(state.deref().clone());
    });

    html! {
        <div class="card-base">
            <div class="mb-6">
                <TextInput id={"email"} input_type={Some(INPUTTYPE::Email)} on_change={email_on_change}/>
                <TextInput id={"password"} input_type={Some(INPUTTYPE::Password)} on_change={password_on_change}/>
            </div>
            <button type="submit" onclick={submit} class="button">{"Submit"}</button>
        </div>
    }
}
