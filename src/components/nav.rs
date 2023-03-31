use yew::prelude::*;
use yew_router::prelude::*;

use crate::services::backend::authenticated;
use crate::Route;

#[function_component]
pub fn Nav() -> Html {
    let logged_in = authenticated();

    let (authentication_route, authentication_label) = match logged_in {
        true => (Route::Logout, "Logout"),
        false => (Route::LoginPage, "Login"),
    };

    html! {
        <nav class="border-gray-200 px-2 px-4 py-2.5 rounded bg-transparent dark:bg-gray-900">
              <ul class="border border-gray-100 text-sm font-medium border-0 bg-transparent dark:border-gray-700">
                <li class="navbar-link">
                    <Link<Route> to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                </li>
                <li class="navbar-link">
                    <Link<Route> to={Route::Messages}>
                        { "Messages" }
                    </Link<Route>>
                </li>
                <li class="navbar-link">
                    <Link<Route> to={Route::Token}>
                        { "Token" }
                    </Link<Route>>
                </li>
                <li class="navbar-link">
                    <Link<Route> to={authentication_route}>
                        { authentication_label }
                    </Link<Route>>
                </li>
                </ul>
        </nav>

    }
}
