use std::rc::Rc;
use web_sys::console::log_1;
use yew::{Callback};
use content::PostPart;
use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use serde::Serialize;
use crate::components::login::Login;
use crate::{content, Route};


#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String
}

fn send_login_request() {
    wasm_bindgen_futures::spawn_local(async move {
        let backend_url = "http://127.0.0.1:8081";
        log_1(&"calling url".to_string().into());
        let response = Request::get(backend_url).send().await.unwrap();
        let msg = format!("returned {}", response.status());
        log_1(&msg.into());
    });
}

#[function_component]
pub fn LoginPage() -> Html {

    let username_static = "asdf".to_string();
    let on_button_clicked = Callback::from(|value: String| {
        let login_request = LoginRequest{ username: value, password: "testpass".to_string() };
        wasm_bindgen_futures::spawn_local(async move {
            let backend_url = "http://127.0.0.1:8081";
            log_1(&"calling url".to_string().into());
            let response = Request::get(backend_url).send().await.unwrap();
            let msg = format!("returned {}", response.status());
            log_1(&msg.into());
        });
    });

    html! {
        <div class="card">
            <Login username={username_static} on_button_clicked={on_button_clicked}/>
        </div>

    }
}

use wasm_bindgen_test::wasm_bindgen_test;
#[wasm_bindgen_test]
fn test_this () {
    send_login_request();
}
