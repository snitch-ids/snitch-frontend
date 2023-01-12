use crate::services::backend::MessageBackend;
use yew::{function_component, html, Html, Properties};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub message: MessageBackend,
}

#[function_component(MessageCard)]
pub fn message_card(props: &Props) -> Html {
    let message = &props.message;
    html!(
        <div class="card">
            <div class="card-content">
                <h2>{ message.hostname.clone() }</h2>
                <p>{ message.title.clone() }</p>
                <p>{ message.timestamp.clone() }</p>
            </div>
        </div>
    )
}
