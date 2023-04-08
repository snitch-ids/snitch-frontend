use crate::pages::login::LoginRequest;
use crate::pages::register::RegisterRequest;
use crate::stores::user_store::UserStore;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use wasm_cookies;
use web_sys::console::log_1;
use web_sys::RequestCredentials;
use yewdux::prelude::*;

const BACKEND_URL: &str = env!("SNITCH_BACKEND_URL"); // See Dockerfile
const USER_COOKIE_NAME: &str = "user_cookie";
pub type MessageToken = String;

#[derive(Serialize)]
pub struct MessagesRequest<'a> {
    pub(crate) hostname: &'a str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserResponse {
    pub(crate) email: String,
}

pub fn register_user(register_request: &RegisterRequest) {
    let request = register_request.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let url = format!("{BACKEND_URL}/register");

        let msg = format!("calling url {url}");
        log_1(&msg.to_string().into());

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

async fn get_user_info() -> Result<UserResponse, FetchError> {
    let url = format!("{BACKEND_URL}/user");
    let msg = format!("calling url {url}");
    log_1(&msg.to_string().into());

    let response = Request::get(&*url)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .unwrap();
    let msg = format!("auth status: {}", response.status());
    log_1(&msg.into());

    response
        .json::<UserResponse>()
        .await
        .map_err(|_e| FetchError::NoMessage)
}

pub fn authenticate(login_request: LoginRequest, dispatch: Dispatch<UserStore>) {
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

        if response.status() == 200 {
            if let Ok(user_info) = get_user_info().await {
                dispatch.set(UserStore {
                    email: Some(user_info.email),
                });
            }
        }

        log_1(&msg.into());
    });
}

pub fn logout(dispatch: Dispatch<UserStore>) {
    wasm_bindgen_futures::spawn_local(async move {
        log_1(&"calling url".to_string().into());
        let url = format!("{BACKEND_URL}/logout");
        let response = Request::post(&*url)
            .header("Accept", "*/*")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .unwrap();
        let msg = format!("logout status: {}", response.status());
        if response.status() == 200 {
            dispatch.set(UserStore { email: None });
        }
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

pub async fn request_hostnames() -> Result<Vec<String>, FetchError> {
    let msg = format!("requesting hostnames");
    log_1(&msg.into());

    let messages_url = format!("{BACKEND_URL}/messages/hostnames");
    let response = Request::get(&*messages_url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .unwrap();

    let msg = format!("requesting done {:?}", response.status());
    log_1(&msg.into());

    response
        .json::<Vec<String>>()
        .await
        .map_err(|_e| FetchError::NoMessage)
}

pub async fn request_messages(hostname: &str) -> Result<Vec<MessageBackend>, FetchError> {
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
        .map_err(|_e| FetchError::NoMessage)
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
        .map_err(|_e| FetchError::NoMessage)
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
        .map_err(|_e| FetchError::NoMessage)
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
