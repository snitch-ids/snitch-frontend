use yew::prelude::*;

use crate::services::backend::test;

#[function_component]
pub fn TEST() -> Html {
    let mut x: String = Default::default();
    test();
    x.push('a');

    html! {
        <div class="card">
            {"hi"}
        {x.clone()}
        </div>
    }
}
