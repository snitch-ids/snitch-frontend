use yew::prelude::*;

use web_sys::console::log_1;
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

use crate::components::message_list::MessageList;
use crate::services::backend::request_hostnames;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props<'a> {
    pub hostname: &'a str,
}

#[function_component]
pub fn Messages() -> Html {
    let hostnames = use_state(|| vec![]);
    let hostname_want = use_state(|| "".to_string());

    {
        let hostnames = hostnames.clone();
        let hostname_want = hostname_want.clone();

        use_effect_with_deps(
            move |_| {
                let hostnames = hostnames.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_hostnames = request_hostnames().await.unwrap_or_default();
                    hostname_want.set(fetched_hostnames[0].clone());
                    hostnames.set(fetched_hostnames);
                    let msg = format!("found {} hostnames", hostnames.len());
                    log_1(&msg.into());
                });
                || ()
            },
            (),
        );
    }

    let hostname_want_cb = hostname_want.clone();

    let cb: Callback<String> = Callback::from(move |name: String| {
        hostname_want_cb.set(name);
    });

    let hostname_buttons = hostnames.iter().map(|hostname| {
        let hostname_cb = hostname.clone();
        let cb_cb = cb.clone();
        let onclick = Callback::from(move |_| {
            cb_cb.emit(hostname_cb.clone());
        });
        html! {
            <p>
                <button class="button border-solid border-1" {onclick}>{ hostname }</button>
            </p>
        }
    });

    html! {
        <>
            <div class="relative overflow-x-auto">
                if { hostnames.len() > 0 } {
                    <div class="flex">
                        { for hostname_buttons }
                    </div>
                    <MessageList hostname={(*hostname_want).clone()}/>
                }
            </div>
        </>
    }
}
