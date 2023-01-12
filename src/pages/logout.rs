use yew::prelude::*;

use crate::services::backend::clear_session_storage;

#[function_component]
pub fn Logout() -> Html {
    clear_session_storage();

    html! {
        <div class="card">
            {"You are logged out now."}
        </div>
    }
}
