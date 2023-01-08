use std::rc::Rc;
use web_sys::console::log_1;
use yew::{Callback};
use content::PostPart;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::login::Login;
use crate::{content, Route};


#[function_component]
pub fn LoginPage() -> Html {
    let username_static = "asdf".to_string();
    let username = use_state(|| "".to_string());
    let username_state = username.clone();
    let on_button_clicked = Callback::from(|_| {
        let greet = "bla".to_string();
        log_1(&greet.into());
    });

    html! {
        <div class="card">
            <Login username={username_static} on_button_clicked={on_button_clicked}/>
        </div>

    }
}
