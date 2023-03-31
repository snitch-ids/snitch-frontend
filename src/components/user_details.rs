use crate::stores::user_store::UserStore;
use yew::function_component;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct UserDetails {
    logged_in: Dispatch<UserStore>,
}

#[function_component]
pub fn UserDetailsComponent() -> Html {
    html!({ "bla" })
}
