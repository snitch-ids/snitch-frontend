use serde::Serialize;
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

// #[function_component(Async)]
// pub fn MessageList() -> Html {
//     let hostname = "admin".to_string();
//     let mes = use_async( async move  {request_messages(hostname).await});
//     let message_vnodes = {
//         let mut cards = (0..ITEMS_PER_PAGE).map(|seed_offset| {
//             html! {
//                 <li class="list-item mb-5">
//                     <MessageCard />
//                 </li>
//             }
//         });
//         html! {
//             <div class="columns">
//                 <div class="column">
//                     <ul class="list">
//                         { for cards }
//                     </ul>
//                 </div>
//             </div>
//         }
//     };
//
//     message_vnodes
// }

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
                    let fetched_messages = request_messages("admin".to_string()).await.unwrap();
                    messages.set(fetched_messages);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <h1>{ "Found messages" }</h1>
            <div>
                <p class="h5">{ for (*messages).iter().map(|m| m.hostname.clone()) }</p>
                <hr />
            </div>
        </>
    }
}
