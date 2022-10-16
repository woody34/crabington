use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonPreset {
    Primary,
    Secondary,
    Tertiary,
}

fn default_button_preset() -> ButtonPreset {
    ButtonPreset::Primary
}

fn default_button_label() -> String {
    "Button".to_string()
}

fn default_button_class() -> String {
    "".to_string()
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_else(default_button_preset)]
    pub preset: ButtonPreset,

    #[prop_or_else(default_button_label)]
    pub label: String,

    #[prop_or_else(default_button_class)]
    pub classes: String,

    pub children: Children
}

#[function_component(Button)]
pub fn button(ButtonProps { preset, label, classes, children }: &ButtonProps) -> Html {
    let mut preset_classes = "";
    match preset {
        &ButtonPreset::Primary => preset_classes = "bg-primary-600 text-white shadow-sm hover:bg-primary-700",
        &ButtonPreset::Secondary => preset_classes = "bg-secondary-100 text-secondary-700 hover:bg-secondary-200",
        &ButtonPreset::Tertiary => preset_classes = "bg-tertiary-100 text-tertiary-700 hover:bg-tertiary-200",
    }

    let base_classes = "m-2 inline-flex items-center rounded-md border border-transparent px-4 py-2 text-sm font-medium";

    let classes = classes!(
        base_classes,
        preset_classes,
        classes.clone()
    );

    html! {
        <button class={classes}>{ label }</button>
    }
}