use yew::prelude::*;
use crate::components::button::button::Button;
use crate::components::button::button::ButtonPreset;

#[function_component(Organisms)]
pub fn organisms() -> Html {
    html! {
        <div class="content">
            <p>{ "Organisms go here!" }</p>
        </div>
    }
}
