use crate::stores::user_store::UserStore;
use yew::function_component;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct UserDetails {
    logged_in: Dispatch<UserStore>,
}

#[function_component]
pub fn UserDetailsComponent() -> Html {
    let (user_state, _) = use_store::<UserStore>();
    let username = &user_state.email;
    html!(
    <div class="card">
        <div class="card-content">
            <div class="card-base">
                <div class="card-title">{"User Details"}</div>
                <div class="text-gray-100">{username}</div>
            </div>
        </div>
        <div class="card-action">
            <button class="button-danger my-10">{"Delete Account"}</button>
        </div>
    </div>
    )
}
