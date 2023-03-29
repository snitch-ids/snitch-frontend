use yew::Callback;

use yew::prelude::*;

use crate::components::login::Login;
use crate::components::login::LoginRequest;
use crate::services::backend::authenticate;

#[function_component]
pub fn LoginPage() -> Html {
    let on_button_clicked = Callback::from(|login_request: LoginRequest| {
        authenticate(login_request);
    });

    html! {
        <div class="grid place-items-center">
            <Login on_button_clicked={on_button_clicked}/>
        </div>
    }
}
