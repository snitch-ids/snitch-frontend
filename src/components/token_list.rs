use serde::Serialize;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

use reqwasm::http::{Request, Response};
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

use crate::components::message::MessageCard;
use crate::components::pagination::{PageQuery, Pagination};
use crate::services::backend::{request_tokens, MessageToken};
use crate::Route;

use crate::services::backend::MessageBackend;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub token: MessageToken,
}

#[function_component(TokenCard)]
pub fn token_card(props: &Props) -> Html {
    let token = &props.token;
    html!(
        <div class="card">
            <div class="card-content">
                <h2>{ token }</h2>
            </div>
        </div>
    )
}

#[function_component]
pub fn TokenList() -> Html {
    log::info!("Fetching token");
    let tokens = use_state(|| vec![]);
    {
        let tokens = tokens.clone();
        use_effect_with_deps(
            move |_| {
                let tokens = tokens.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_tokens = request_tokens().await.unwrap_or_default();
                    tokens.set(fetched_tokens);
                });
                || ()
            },
            (),
        );
    }

    let mut token_cards = tokens.iter().map(|token| {
        html! {
            <li class="list-item mb-5">
                <TokenCard token={token.clone()}/>
            </li>
        }
    });

    html! {
        <>
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        { for token_cards }
                    </ul>
                </div>
            </div>
        </>
    }
}
