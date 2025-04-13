use serde::Serialize;
use std::ops::Deref;
use yew::prelude::*;
use yew::Callback;
use yewdux::prelude::*;

use crate::components::atomics::text_input::{TextInput, INPUTTYPE};
use crate::services::backend::authenticate;
use crate::stores::user_store::UserStore;

#[derive(Serialize, Debug, Default, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[function_component]
pub fn LoginPage() -> Html {
    let (counter, dispatch) = use_store::<UserStore>();
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

    let submit = Callback::from(move |_| {
        authenticate(state.deref().clone(), dispatch.clone());
    });

    html! {
        <div class="grid place-items-center">
            <div class="card-base my-10">
                <div class="my-2">
                    <TextInput id={"email"} placeholder={Some("email")} input_type={Some(INPUTTYPE::Email)} on_change={email_on_change}/>
                    <TextInput id={"password"} placeholder={Some("*********")} input_type={Some(INPUTTYPE::Password)} on_change={password_on_change}/>
                </div>
                <button type="submit" onclick={submit} class="button">{"Login"}</button>
            </div>
        </div>
    }
}
