use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::home::Home;
use crate::pages::login::LoginPage;
use crate::pages::logout::Logout;
use crate::pages::message_list::MessageList;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::token::Token;

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
    #[at("/logout")]
    Logout,
    #[at("/token")]
    Token,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="bg-white dark:bg-gray-900 min-h-screen">
        <BrowserRouter>
            <Nav />
            <main>
                <div class="flex mb-4">
                    <div class="w-1/6"></div>
                    <div class="container w-2/3 mt-6">
                        <Switch<Route> render={switch} />
                    </div>
                    <div class="w-1/6"></div>
                </div>
            </main>
        </BrowserRouter>
        </div>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Nav />
            <main>
                <Switch<Route> render={switch} />
            </main>
        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Messages => {
            html! { <MessageList /> }
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
        Route::Logout => {
            html! { <Logout /> }
        }
        Route::Token => {
            html! { <Token /> }
        }
    }
}
