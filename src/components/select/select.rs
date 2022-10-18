use yew::prelude::*;
use yew_hooks::*;
use crate::hooks::use_show::*;
use log::{ info };

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub options: Vec<String>,

    #[prop_or_default]
    pub selected: String,

    #[prop_or_default]
    pub on_select: Callback<String>
}

#[function_component(Select)]
pub fn select(Props { options, selected, on_select }: &Props) -> Html {
    let (show, toggle_show) = use_show();
    
    let node = use_node_ref();
    let show_click_away = show.clone();
    use_click_away(node.clone(), move |_: Event| {
        show_click_away.set(false);
    });

    let transition_classes = use_state(|| String::from(""));
    let classes = classes!(
        String::from("absolute right-0 z-10 mt-2 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 w-full"),
        (*show).clone().then(|| Some("show"))
    );

    let transition_start_classes = transition_classes.clone();
    let on_transition_start = Callback::from(move |_: TransitionEvent| {
        info!("start");
        transition_start_classes.set("start".to_string());
    });

    let transition_ent_classes = transition_classes.clone();
    let on_transition_end = Callback::from(move |_: TransitionEvent| {
        info!("end");
        transition_ent_classes.set("Legs".to_string());
    });

    html! {
        <div ref={node} class="relative inline-block text-left w-full">
            <div>
                <button
                    type="button"
                    class="inline-flex w-full justify-between rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50"
                    onclick={toggle_show}
                >
                    {selected}
                    <i class="fa-thin fa-angle-down -mr-1 ml-2 pt-[2px] h-4 w-4"></i>
                </button>
            </div>

            if *show && options.len() > 0 {
                <div class={classes} id="dropdown" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" ontransitionrun={on_transition_start} ontransitioncancel={on_transition_end}>
                    <div class="py-1" role="none">
                        {
                            options.iter().map(|option| {
                                let callback = on_select.clone();
                                let emit_option = option.clone();
                                let show = show.clone();
                                let on_click = Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    callback.emit(emit_option.to_string());
                                    show.set(false);
                                });
                                let active_option = selected == option;
                                let classes = classes!(
                                    String::from("text-gray-700 block px-4 py-2 text-sm hover:bg-gray-100 text-gray-900 animate-fade-in"),
                                    active_option.then(|| Some("bg-gray-100 text-gray-900 font-bold"))
                                );
                                html! {
                                    <a class={classes} role="menuitem" onclick={on_click}>{ option }</a>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            }
      </div>
    }
}