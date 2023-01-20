use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::author::Author;
use crate::pages::author_list::AuthorList;
use crate::pages::home::Home;
use crate::pages::login::LoginPage;
use crate::pages::logout::Logout;
use crate::pages::message_list::MessageList;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::post::Post;
use crate::pages::post_list::PostList;
use crate::pages::test::TEST;
use crate::pages::token::Token;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/messages/")]
    Messages,
    #[at("/posts/:id")]
    Post { id: u32 },
    #[at("/posts")]
    Posts,
    #[at("/authors/:id")]
    Author { id: u32 },
    #[at("/authors")]
    Authors,
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
    #[at("/TEST")]
    TEST,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        </BrowserRouter>
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
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Post { id } => {
            html! { <Post seed={id} /> }
        }
        Route::Posts => {
            html! { <PostList /> }
        }
        Route::Messages => {
            html! { <MessageList /> }
        }
        Route::Author { id } => {
            html! { <Author seed={id} /> }
        }
        Route::Authors => {
            html! { <AuthorList /> }
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
        Route::TEST => {
            html! { <TEST /> }
        }
    }
}
