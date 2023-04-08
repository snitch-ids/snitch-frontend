use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct UserStore {
    pub(crate) email: Option<String>,
}
