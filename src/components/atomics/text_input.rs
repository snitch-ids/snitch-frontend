use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub on_change: Callback<String>

}

#[function_component]
pub fn TextInput(props: &Props) -> Html {
    let state = use_state(String::new);

    let onchange = Callback::from(move |event:Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        props.on_change.emit(value);
    });
    html! {
        <input type="text" onchange={onchange} id={props.id.clone()} class="input" placeholder="username" required=true />
    }
}