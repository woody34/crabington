use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::components::button::button::Button;
use crate::components::button::button::ButtonPreset;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="content">
            <p>{ "Home" }</p>
        </div>
    }
}
