use yew::prelude::*;
use yew_hooks::*;
use crate::components::nav_mobile::*;

#[function_component(Header)]
pub fn header() -> Html {
    let show = use_state_eq(|| false);
    let on_show = {
        let show = show.clone();
        Callback::from(move |_: MouseEvent| show.set(!*show))
    };


    let node = use_node_ref();
    // let show_click_away = show.clone();
    // use_click_away(node.clone(), move |_: Event| {
    //     if *show_click_away {
    //         show_click_away.set(false);
    //     }
    // });

    html! {
        <header class="bg-white shadow-sm shadow-primary-200 lg:static lg:overflow-y-visible mt-4">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                <div class="relative flex justify-between lg:gap-8 xl:grid xl:grid-cols-12">
                    <div class="flex md:absolute md:inset-y-0 md:left-0 lg:static xl:col-span-2">
                    <div class="flex flex-shrink-0 items-center">
                        <a href="#">
                            <img class="block h-8 w-auto" src="https://rustacean.net/assets/rustacean-flat-gesture.png" alt="Crabington Components" />
                        </a>
                    </div>
                    </div>
                    <div class="min-w-0 flex-1 md:px-8 lg:px-0 xl:col-span-6">
                        <div class="flex items-center px-6 py-4 md:mx-auto md:max-w-3xl lg:mx-0 lg:max-w-none xl:px-0">
                            // <div class="w-full">
                            //     <label for="search" class="sr-only">{"Search"}</label>
                            //     <div class="relative">
                            //         <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
                            //         // <!-- Heroicon name: mini/magnifying-glass -->
                            //         <svg class="h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            //             <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z" clip-rule="evenodd" />
                            //         </svg>
                            //         </div>
                            //         <input id="search" name="search" class="block w-full rounded-md border border-gray-300 bg-white py-2 pl-10 pr-3 text-sm placeholder-gray-500 focus:border-rose-500 focus:text-gray-900 focus:placeholder-gray-400 focus:outline-none focus:ring-1 focus:ring-rose-500 sm:text-sm" placeholder="Search" />
                            //     </div>
                            // </div>
                        </div>
                    </div>
                    <div class="flex items-center md:absolute md:inset-y-0 md:right-0 lg:hidden">
                        // <!-- Mobile menu button -->
                        <button
                            type="button"
                            class="inline-flex items-center justify-center rounded-md p-2 text-gray-400 active:bg-gray-200 hover:bg-gray-100 hover:text-gray-500 mb-1"
                            onclick={on_show}
                            ref={node}
                        >
                            <span class="sr-only">{"Open menu"}</span>
                            <em class="fa-solid fa-bars"></em>
                        </button>
                    </div>
                    <div class="hidden lg:flex lg:items-center lg:justify-end xl:col-span-4">
                        <div class="flex flex-shrink-0 items-center -mt-1">
                            <a href="https://github.com/woody34/crabington" target="_blank"><i class="fa-brands fa-github fa-2xl"></i></a>
                        </div>   
                    </div>
                </div>
            </div>

            // <!-- Mobile menu, show/hide based on menu state. -->
            <nav class="lg:hidden" aria-label="Global">
                if *show {
                    <div id="mobile-menu" class={classes!((*show).clone().then(|| Some("show")), "animate-fade-in", "relative", "top-0")}>
                        <NavMobile />
                    </div>
                }
            </nav>
        </header>
    }
}
