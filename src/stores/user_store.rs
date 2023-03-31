use yewdux::prelude::*;

#[derive(Store, Default, PartialEq)]
pub struct UserStore {
    pub logged_in: bool,
}
