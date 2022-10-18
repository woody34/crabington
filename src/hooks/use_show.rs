use yew::prelude::*;

pub fn use_show() -> (UseStateHandle<bool>, Callback<MouseEvent>) {
    let show = use_state_eq(|| false);
    let toggle_show = {
        let show = show.clone();
        Callback::from(move |_: MouseEvent| show.set(!*show))
    };
    (show, toggle_show)
}