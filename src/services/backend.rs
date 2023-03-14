use crate::components::login::LoginRequest;
use crate::pages::message_list::MessagesRequest;
use crate::pages::register::RegisterRequest;
use reqwasm::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fmt::format;
use wasm_cookies;
use web_sys::console::log_1;
use web_sys::RequestCredentials;
use std::env;

const BACKEND_URL: &str = env!("SNITCH_BACKEND_URL");
const USER_COOKIE_NAME: &str = "user_cookie";
pub type MessageToken = String;

pub fn register_user(register_request: &RegisterRequest) {
    let request = register_request.clone();
    wasm_bindgen_futures::spawn_local(async move {
        log_1(&"calling url".to_string().into());
        let url = format!("{BACKEND_URL}/register");
        let payload = to_string(&request).unwrap();
        let response = Request::post(&*url)
            .header("Content-Type", "application/json")
            .header("Accept", "*/*")
            .credentials(RequestCredentials::Include)
            .body(&payload)
            .send()
            .await
            .unwrap();
        let msg = format!("auth status: {}", response.status());
        log_1(&msg.into());
    });
}

pub fn authenticate(login_request: LoginRequest) {
    wasm_bindgen_futures::spawn_local(async move {
        log_1(&"calling url".to_string().into());
        let login_url = format!("{BACKEND_URL}/login");
        let payload = to_string(&login_request).unwrap();
        let response = Request::post(&*login_url)
            .header("Content-Type", "application/json")
            .header("Accept", "*/*")
            .credentials(RequestCredentials::Include)
            .body(&payload)
            .send()
            .await
            .unwrap();
        let msg = format!("auth status: {}", response.status());
        log_1(&msg.into());
    });
}

pub fn logout() {
    wasm_bindgen_futures::spawn_local(async move {
        log_1(&"calling url".to_string().into());
        let url = format!("{BACKEND_URL}/logout");
        let response = Request::post(&*url)
            .header("Content-Type", "application/json")
            .header("Accept", "*/*")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .unwrap();
        let msg = format!("logout status: {}", response.status());
        log_1(&msg.into());
    });
}

pub fn authenticated() -> bool {
    wasm_cookies::get(USER_COOKIE_NAME).is_some()
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
    let payload = to_string(&messages_request).unwrap();
    let messages_url = format!("{BACKEND_URL}/messages/all/");
    let response = Request::post(&*messages_url)
        .credentials(RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .body(&payload)
        .send()
        .await
        .unwrap();

    response
        .json::<Vec<MessageBackend>>()
        .await
        .map_err(|e| FetchError::NoMessage)
}

pub fn test() {
    wasm_bindgen_futures::spawn_local(async move {
        let messages_url = format!("{BACKEND_URL}/");
        let msg = format!("requesting data for hostname {}", messages_url);
        log_1(&msg.into());
        let request = Request::get(&*messages_url).credentials(RequestCredentials::Include);
        request.send().await.unwrap();
    })
}

pub async fn create_token() -> MessageToken {
    log_1(&"want token".into());
    let messages_url = format!("{BACKEND_URL}/token/new");
    let response = Request::get(&*messages_url)
        .credentials(RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();
    let token = response
        .json::<MessageToken>()
        .await
        .map_err(|e| FetchError::NoMessage)
        .expect("TODO: panic token");
    token
}

pub async fn request_tokens() -> Result<Vec<MessageToken>, FetchError> {
    log_1(&"want token".into());
    let messages_url = format!("{BACKEND_URL}/token");
    let response = Request::get(&*messages_url)
        .credentials(RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();
    response
        .json::<Vec<MessageToken>>()
        .await
        .map_err(|e| FetchError::NoMessage)
}
