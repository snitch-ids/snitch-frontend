use crate::services::backend::create_token;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Token() -> Html {
    let create_token = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            create_token().await;
        })
    });

    html!(
        <div class="card">
            <div class="card-content">
                <button onclick={create_token}>
                    { "create token" }
                </button>
            </div>
        </div>
    )
}
