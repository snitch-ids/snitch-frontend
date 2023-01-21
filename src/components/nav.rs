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
        <nav class="bg-white border-gray-200 px-2 sm:px-4 py-2.5 rounded dark:bg-gray-900">
            <div class="container flex flex-wrap items-center justify-between mx-auto">
                <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                  <ul class="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
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
                        <Link<Route> to={authentication_route}>
                            { authentication_label }
                        </Link<Route>>
                    </li>
                    <li class="navbar-link">
                        <Link<Route> to={Route::Token}>
                            { "Token" }
                        </Link<Route>>
                    </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
