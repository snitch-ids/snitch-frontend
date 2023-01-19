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
use wasm_cookies;
use wasm_cookies::{CookieOptions, SameSite};
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
            .header("Accept", "*/*")
            .credentials(RequestCredentials::Include)
            .body(&payload)
            .send()
            .await
            .unwrap();
        let msg = format!("returned {}", response.status());
        log_1(&msg.into());
        // let response_json = response.json::<LoginResponse>().await.unwrap();
        // storage::LocalStorage::set("refresh_token", response_json.refresh_token.clone())
        //     .expect("TODO: panic message");
        // storage::LocalStorage::set("access_token", response_json.access_token.clone())
        //     .expect("TODO: panic message");

        // let mut cookie_options = CookieOptions::default().with_domain("127.0.0.1:8081").with_path("/messages/all/").with_same_site(SameSite::None);

        // let access_token =  &response_json.access_token;
        // let mut act: String = "".to_string();
        // for x in access_token.split("access_token=") {
        //     act = x.to_string();
        // }
        // wasm_cookies::set_raw("access_tokenxx", &act, &cookie_options);
        // let c = wasm_cookies::set_raw(
        //     "refresh_token",
        //     &response_json.refresh_token,
        //     &cookie_options,
        // );
        // let msg = format!("{c:?}");
        // log_1(&msg.into());
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

    let payload = to_string(&messages_request).unwrap();

    let messages_url = format!("{BACKEND_URL}/messages/all/");
    let request = Request::post(&*messages_url)
        .credentials(RequestCredentials::Include)
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

pub fn clear_session_storage() {}

pub fn authenticated() -> bool {
    storage::LocalStorage::get::<String>("access_token").is_ok()
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
