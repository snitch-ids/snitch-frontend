use gloo::storage;
use gloo::storage::Storage;
use reqwasm;
use reqwasm::http::{Headers, Request};
use serde::Serialize;
use serde_json::to_string;
use std::fmt::format;
use wasm_cookies::CookieOptions;
use web_sys;
use web_sys::console::log_1;
use web_sys::window;

#[derive(Serialize)]
pub struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

const BACKEND_URL: &str = "http://127.0.0.1:8081/login";

pub fn authenticate(login_request: LoginRequest) {
    wasm_bindgen_futures::spawn_local(async move {
        storage::LocalStorage::set("a", "b").expect("TODO: panic message");

        log_1(&"calling url".to_string().into());
        let payload = to_string(&login_request).unwrap();
        let mut headers = Headers::new();
        headers.append("Content-Type", "application/json");
        let response = Request::post(BACKEND_URL)
            .headers(headers)
            .body(&payload)
            .send()
            .await
            .unwrap();
        let msg = format!("{:?}", response.headers());
        log_1(&msg.into());
        let msg = format!("returned {}", response.status());
        log_1(&msg.into());
    });
}
