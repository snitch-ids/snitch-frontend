use crate::components::login::{LoginRequest, LoginResponse};
use crate::pages::message_list::MessagesRequest;
use gloo::file::futures;
use gloo::storage;
use gloo::storage::Storage;
use reqwasm;
use reqwasm::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fmt::format;
use wasm_cookies::set_raw as set_cookie;
use wasm_cookies::CookieOptions;
use web_sys::console::log_1;
use web_sys::RequestCredentials;
use yew::{html, use_state, Properties};

const BACKEND_URL: &str = "http://127.0.0.1:8081";

pub fn authenticate(login_request: LoginRequest) {
    wasm_bindgen_futures::spawn_local(async move {
        log_1(&"calling url".to_string().into());
        let login_url = format!("{BACKEND_URL}/login");
        let payload = to_string(&login_request).unwrap();
        let mut headers = Headers::new();
        headers.append("Content-Type", "application/json");
        let response = Request::post(&*login_url)
            // .headers(headers)
            .header("Content-Type", "application/json")
            .body(&payload)
            .send()
            .await
            .unwrap();
        let msg = format!("returned {}", response.status());
        log_1(&msg.into());
        let response_json = response.json::<LoginResponse>().await.unwrap();
        storage::LocalStorage::set("refresh_token", response_json.refresh_token.clone())
            .expect("TODO: panic message");
        storage::LocalStorage::set("access_token", response_json.access_token.clone())
            .expect("TODO: panic message");

        let mut cookie_options = CookieOptions::default().with_path("/messages/all/");

        set_cookie("access_token", &response_json.access_token, &cookie_options);
        let c = set_cookie(
            "refresh_token",
            &response_json.refresh_token,
            &cookie_options,
        );
        let msg = format!("{c:?}");
        log_1(&msg.into());
    });
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct MessageBackend {
    pub hostname: String,
    pub title: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Clone, Debug)]
pub enum FetchError {
    NoMessage,
}

pub async fn request_messages(hostname: String) -> Result<Vec<MessageBackend>, FetchError> {
    let msg = format!("requesting data for hostname {}", hostname);
    log_1(&msg.into());
    let messages_request = MessagesRequest { hostname };
    let access_token: String = storage::LocalStorage::get("access_token").unwrap_or_default();
    // let access_token = access_token.split(";").next().unwrap();

    let refresh_token: String = storage::LocalStorage::get("refresh_token").unwrap_or_default();
    // let refresh_token = refresh_token.split(";").next().unwrap();
    let cookie = format!("{refresh_token}; {access_token}");
    use wasm_bindgen::JsCast;

    // let window = web_sys::window().unwrap();
    // let document = window.document().unwrap();
    // let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    // let cookie = html_document.cookie().unwrap();

    let payload = to_string(&messages_request).unwrap();

    let headers = Headers::new();
    let messages_url = format!("{BACKEND_URL}/messages/all/");
    headers.append("Content-Type", "application/json");
    headers.append("Cookie", &cookie);
    let request = Request::post(&*messages_url)
        .credentials(RequestCredentials::Include)
        // .headers(headers)
        .header("Content-Type", "application/json")
        .body(&payload);

    let msg = format!("{:?} {cookie}", request);
    log_1(&msg.into());
    let response = request.send().await.unwrap();

    let data = response
        .json::<Vec<MessageBackend>>()
        .await
        .map_err(|e| FetchError::NoMessage);
    data
}

pub fn clear_session_storage() {
    storage::LocalStorage::delete("access_token");
}

pub fn authenticated() -> bool {
    storage::LocalStorage::get::<String>("access_token").is_ok()
}
