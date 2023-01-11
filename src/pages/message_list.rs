use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;

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
    let hostname = "admin".to_string();
    request_messages(hostname);

    let message_vnodes = {
        let mut cards = (0..ITEMS_PER_PAGE).map(|seed_offset| {
            html! {
                <li class="list-item mb-5">
                    <MessageCard />
                </li>
            }
        });
        html! {
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        { for cards }
                    </ul>
                </div>
            </div>
        }
    };

    message_vnodes
}
