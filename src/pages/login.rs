use web_sys::console::log_1;
use yew::Callback;

use yew::prelude::*;

use crate::components::login::Login;
use crate::components::login::LoginRequest;
use crate::services::backend::authenticate;
use reqwasm::http::Request;
use serde::Serialize;

#[function_component]
pub fn LoginPage() -> Html {
    let on_button_clicked = Callback::from(|username: String| {
        let login_request = LoginRequest {
            username: username,
            password: "xxxx".to_string(),
        };
        authenticate(login_request);
    });

    html! {
        <div class="card">
            <Login on_button_clicked={on_button_clicked}/>
        </div>

    }
}
