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
use web_sys::console::log_1;
use yew::{use_state, Properties};

const BACKEND_URL: &str = "http://127.0.0.1:8081";

pub fn authenticate(login_request: LoginRequest) {
    wasm_bindgen_futures::spawn_local(async move {
        log_1(&"calling url".to_string().into());
        let login_url = format!("{BACKEND_URL}/login");
        let payload = to_string(&login_request).unwrap();
        let mut headers = Headers::new();
        headers.append("Content-Type", "application/json");
        let response = Request::post(&*login_url)
            .headers(headers)
            .body(&payload)
            .send()
            .await
            .unwrap();
        let msg = format!("returned {}", response.status());
        log_1(&msg.into());
        let response_json = response.json::<LoginResponse>().await.unwrap();
        storage::LocalStorage::set("refresh_token", response_json.refresh_token)
            .expect("TODO: panic message");
        storage::LocalStorage::set("access_token", response_json.access_token)
            .expect("TODO: panic message");
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
    let cookie: String = storage::LocalStorage::get("access_token").unwrap_or_default();

    let payload = to_string(&messages_request).unwrap();

    let headers = Headers::new();
    let messages_url = format!("{BACKEND_URL}/messages/all/");
    headers.append("Content-Type", "application/json");
    headers.append("Authorization", &cookie);
    let response = Request::post(&*messages_url)
        .headers(headers)
        .body(&payload)
        .send()
        .await
        .unwrap();
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
