use yew::prelude::*;

use crate::services::backend::logout;

#[function_component]
pub fn Logout() -> Html {
    logout();

    html! {
        <div class="card">
            {"You are logged out now."}
        </div>
    }
}
