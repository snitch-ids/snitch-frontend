use serde::Serialize;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

use reqwasm::http::{Request, Response};
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

use crate::components::message::MessageCard;
use crate::components::pagination::{PageQuery, Pagination};
use crate::services::backend::{request_messages, MessageBackend};
use crate::Route;

const ITEMS_PER_PAGE: u32 = 10;
const TOTAL_PAGES: u32 = u32::MAX / ITEMS_PER_PAGE;

#[derive(Serialize)]
pub struct MessagesRequest {
    pub(crate) hostname: String,
}

#[function_component]
pub fn MessageList() -> Html {
    log::info!("Fetching data");
    let messages = use_state(|| vec![]);
    {
        let messages = messages.clone();
        use_effect_with_deps(
            move |_| {
                let messages = messages.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_messages = request_messages("admin".to_string())
                        .await
                        .unwrap_or_default();
                    messages.set(fetched_messages);
                });
                || ()
            },
            (),
        );
    }

    let mut message_cards = messages.iter().map(|message| {
        html! {
            <li class="list-item mb-5">
                <MessageCard message={message.clone()}/>
            </li>
        }
    });

    html! {
        <>
            <h1>{ "Found messages" }</h1>
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        { for message_cards }
                    </ul>
                </div>
            </div>
        </>
    }
}
