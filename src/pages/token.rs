use crate::components::token_list::TokenList;
use crate::services::backend::create_token;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Token() -> Html {
    let create_token_callback = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            create_token().await;
        })
    });
    html!(
        <div class="card">
            <div class="card-content">
                <TokenList />
                <button class="btn" onclick={create_token_callback}>
                    { "create token" }
                </button>
            </div>
        </div>
    )
}
