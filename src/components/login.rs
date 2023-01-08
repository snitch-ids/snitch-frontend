use std::ops::Deref;
use std::rc::Rc;
use yew::{Callback};
use content::PostPart;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

use crate::generator::Generated;
use crate::{content, Route};


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub username: String,
    pub on_button_clicked: Callback<String>
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
        // web_sys::console::log_1(&username.into());
        onclick.emit(username);
    });

    html! {
        // No properties
        <div class="card">
            <div> {"username"}
                <input type="text" name={props.username.clone()} onchange={onchange}/>
            </div>
            <button onclick={submit}>
                { "login" }
            </button>
        </div>

    }
}
