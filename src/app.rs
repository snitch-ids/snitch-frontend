use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::home::Home;
use crate::pages::login::LoginPage;
use crate::pages::logout::Logout;
use crate::pages::messages::Messages;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::register::{AfterRegister, Register};
use crate::pages::token::Token;
use crate::pages::terms::Terms;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/messages/")]
    Messages,
    #[at("/login")]
    LoginPage,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/token")]
    Token,
    #[at("/logout")]
    Logout,
    #[at("/register")]
    Register,
    #[at("/afterregister")]
    AfterRegister,
    #[at("/terms")]
    Terms,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="bg-white dark:bg-gray-900 min-h-screen">
            <BrowserRouter>
                <div class="flex">
                    <div class="flex-none w-34">
                        <Nav/>
                    </div>
                    <div class="flex-auto p-3">
                        <main>
                            <Switch<Route> render={switch} />
                        </main>
                    </div>
                </div>
            </BrowserRouter>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Messages => {
            html! { <Messages /> }
        }
        Route::LoginPage => {
            html! { <LoginPage /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::Token => {
            html! { <Token /> }
        }
        Route::Logout => {
            html! { <Logout /> }
        }
        Route::Register => {
            html! { <Register /> }
        }
        Route::AfterRegister => {
            html! { <AfterRegister /> }
        }
        Route::Terms => {
            html! { <Terms /> }
        }
    }
}
