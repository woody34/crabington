use yew::{prelude::*, html::IntoPropValue};
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::hooks::use_active_route::*;

use log::{info, trace, warn};

/// Nav component
#[function_component(NavMobile)]
pub fn nav_mobile() -> Html {
    let active_route_class = "bg-primary-200 text-primary-900 group flex items-center px-3 py-2 text-2xl font-medium rounded-md transition ease-in-out delay-150";
    let inactive_route_class = "text-primary-700 hover:bg-primary-50 group flex items-center px-3 py-2 text-2xl font-medium rounded-md transition ease-in-out delay-150";
    let active_path = use_active_route();

    html! {
        <div class="mx-auto max-w-3xl space-y-1 px-2 pt-2 pb-3 sm:px-4 transition ease-in-out delay-150">
            // <!-- Current: "bg-gray-100 text-gray-900", Default: "hover:bg-gray-50" -->
            <Link<AppRoute> to={AppRoute::Home}>
                <a class={ if active_path == "/".to_string() { active_route_class } else { inactive_route_class } } aria-current="page">
                    <span class="flex flex-row items-center justify-center">
                        <i class="fa-regular fa-house-chimney-window fa-xl text-primary-400 group-hover:text-primary-500 flex-shrink-0 transition ease-in-out delay-150 w-10"></i>
                        <span class="truncate">{"Home"}</span>
                    </span>
                </a>
            </Link<AppRoute>>

            <Link<AppRoute> to={AppRoute::Atoms}>
                <a class={ if active_path == "/atoms".to_string() { active_route_class } else { inactive_route_class } } aria-current="page">
                    <span class="flex flex-row items-center justify-center">
                        <i class="fa-regular fa-atom-simple fa-xl scale-95 pl-0.5 text-primary-400 group-hover:text-primary-500 flex-shrink-0 transition ease-in-out delay-150 w-10"></i>
                        <span class="truncate">{"Atoms"}</span>
                    </span>
                </a>
            </Link<AppRoute>>

            <Link<AppRoute> to={AppRoute::Molecules}>
                <a class={ if active_path == "/molecules".to_string() { active_route_class } else { inactive_route_class } }>
                    <span class="flex flex-row items-center justify-center">
                        <i class="fa-regular fa-chart-network fa-xl scale-90 text-primary-400 group-hover:text-primary-500 flex-shrink-0 transition ease-in-out delay-150 w-10"></i>
                        <span class="truncate">{"Molecules"}</span>
                    </span>
                </a>
            </Link<AppRoute>>

            <Link<AppRoute> to={AppRoute::Organisms}>
                <a class={ if active_path == "/organisms".to_string() { active_route_class } else { inactive_route_class } }>
                    <span class="flex flex-row items-center justify-center">
                        <i class="fa-regular fa-bacterium fa-xl text-primary-400 pl-0.5 group-hover:text-primary-500 flex-shrink-0 transition ease-in-out delay-150 w-10"></i>
                        <span class="truncate">{"Organisms"}</span>
                    </span>
                </a>
            </Link<AppRoute>>
            <div class="flex justify-center">
                <div><a href="https://github.com/woozdy34/crabington" target="_blank" class="px-3 py-2"><i class="fa-brands fa-github fa-2xl"></i></a></div>
            </div>
        </div>
    }
}
