use serde::Serialize;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

use reqwasm::http::{Request, Response};
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

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
        <>
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                { token }
            </th>
            <td class="px-6 py-4">
                <a href="#" class="font-medium text-red-600 dark:text-red-500 hover:underline">{"Revoke"}</a>
            </td>
        </>
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
            <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                <TokenCard token={token.clone()}/>
            </tr>
        }
    });

    html! {
        <>
            <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                    <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                        <tr>
                            <th scope="col" class="px-6 py-3">
                                {"Token"}
                            </th>
                            <th scope="col" class="px-6 py-3">
                                {"Action"}
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        { for token_cards }
                    </tbody>
                </table>
            </div>
        </>
    }
}
