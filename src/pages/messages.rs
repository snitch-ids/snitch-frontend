use serde::Serialize;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

use reqwasm::http::{Request, Response};
use web_sys::console::log_1;
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

use crate::components::message_list::MessageList;
use crate::services::backend::{request_hostnames, request_messages, MessageBackend};
use crate::Route;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props<'a> {
    pub hostname: &'a str,
}

#[function_component]
pub fn Messages() -> Html {
    let hostnames = use_state(|| vec![]);
    {
        let hostnames = hostnames.clone();
        use_effect_with_deps(
            move |_| {
                let hostnames = hostnames.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_hostnames = request_hostnames().await.unwrap_or_default();
                    hostnames.set(fetched_hostnames);
                    let msg = format!("found {} hostnames", hostnames.len());
                    log_1(&msg.into());
                });
                || ()
            },
            (),
        );
    }

    let button = hostnames.iter().map(|hostname| {
        html! {
            { hostname }
        }
    });

    html! {
        <>
            <div class="relative overflow-x-auto">
                <button> { for button } </button>
                if { hostnames.len() > 0 } {
                    <MessageList hostname={hostnames[0].clone()}/>
                }
            </div>
        </>
    }
}
