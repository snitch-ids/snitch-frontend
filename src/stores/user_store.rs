use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum AuthenticationError {
    LoginFailed,
}

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct UserStore {
    pub(crate) email: Option<String>,
    pub(crate) authentication_error: Option<AuthenticationError>,
}
