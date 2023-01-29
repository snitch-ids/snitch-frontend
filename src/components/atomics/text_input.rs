use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub on_change: Callback<String>,
}

#[function_component]
pub fn TextInput(props: &Props) -> Html {
    let state = use_state(String::new);
    let onchange = {
        let emit_on_change = props.on_change.clone();
        let state_change_handle = state.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            emit_on_change.emit(value.clone());
            state_change_handle.set(value);
        })
    };
    html! {
        <input type="text" onchange={onchange} id={props.id.clone()} class="input" placeholder={props.id.clone()} required=true />
    }
}
