use serde::Serialize;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

use reqwasm::http::{Request, Response};
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

use crate::components::message::MessageCard;
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
            <MessageCard message={message.clone()}/>
        }
    });

    html! {
        <>
            <div class="relative overflow-x-auto">
                <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                    <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                        <tr>
                            <th scope="col" class="px-6 py-3">
                                {"Product name"}
                            </th>
                            <th scope="col" class="px-6 py-3">
                                {"Color"}
                            </th>
                            <th scope="col" class="px-6 py-3">
                                {"Category"}
                            </th>
                            <th scope="col" class="px-6 py-3">
                                {"Price"}
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        { for message_cards }
                    </tbody>
                </table>
            </div>
        </>
    }
}
