use yew::prelude::*;
use crate::components::button::button::Button;
use crate::components::button::button::ButtonPreset;

#[function_component(Molecules)]
pub fn molecules() -> Html {
    html! {
        <div class="content">
            <p>{ "Molecules go here!" }</p>
        </div>
    }
}
