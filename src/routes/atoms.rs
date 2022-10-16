use yew::prelude::*;
use crate::components::button::button::Button;
use crate::components::button::button::ButtonPreset;

#[function_component(Atoms)]
pub fn atoms() -> Html {
    html! {
        <div class="content">
            <Button preset={ButtonPreset::Primary} label="Primary">{ "test" }</Button>
            <Button preset={ButtonPreset::Secondary} label="Secondary">{ "test" }</Button>
            <Button preset={ButtonPreset::Tertiary} label="Tertiary">{ "test" }</Button>
        </div>
    }
}
