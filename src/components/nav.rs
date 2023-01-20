use yew::prelude::*;
use yew_router::prelude::*;

use crate::services::backend::authenticated;
use crate::Route;

#[function_component]
pub fn Nav() -> Html {
    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let active_class = if !*navbar_active { "is-active" } else { "" };

    let logged_in = authenticated();

    let (authentication_route, authentication_label) = match logged_in {
        true => (Route::Logout, "Logout"),
        false => (Route::LoginPage, "Login"),
    };

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>

                <button class={classes!("navbar-burger", "burger", active_class)}
                    aria-label="menu" aria-expanded="false"
                    onclick={toggle_navbar}
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>
            <div class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Posts}>
                        { "Posts" }
                    </Link<Route>>

                    <div class="navbar-item has-dropdown is-hoverable">
                        <div class="navbar-link">
                            { "More" }
                        </div>
                        <div class="navbar-dropdown">
                            <Link<Route> classes={classes!("navbar-item")} to={Route::Authors}>
                                { "Meet the authors" }
                            </Link<Route>>
                        </div>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Messages}>
                            { "Messages" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={authentication_route}>
                            { authentication_label }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Token}>
                            { "Token" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::TEST}>
                            { "TEST" }
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </nav>
    }
}
