use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonPreset {
    Primary,
    Secondary
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
    let primary_classes: &str = "bg-indigo-600 text-white shadow-sm hover:bg-indigo-700";
    let secodary_classes: &str = "bg-indigo-100 text-indigo-700 hover:bg-indigo-200";
    let preset_classes: &str = if preset == &ButtonPreset::Primary { primary_classes } else { secodary_classes };

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