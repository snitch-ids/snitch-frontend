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
        //     <th class="card-content">
        //         <h2>{ message.hostname.clone() }</h2>
        //         <p>{ message.title.clone() }</p>
        //         <p>{ message.timestamp.clone() }</p>
        // </th>
                <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                    { message.hostname.clone() }
                </th>
                <td class="px-6 py-4">
                    { message.title.clone() }
                </td>
                <td class="px-6 py-4">
                    { message.timestamp.clone() }
                </td>
                <td class="px-6 py-4">
                   { message.timestamp.clone() }
                </td>
            </tr>
    )
}
