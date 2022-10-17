use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::hooks::use_active_route::use_active_route;

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    let active_route_class = "bg-primary-200 text-primary-900 group flex items-center px-3 py-2 text-sm font-medium rounded-md transition ease-in-out delay-150";
    let inactive_route_class = "text-primary-700 hover:bg-primary-50 group flex items-center px-3 py-2 text-sm font-medium rounded-md transition ease-in-out delay-150";
    let active_path = use_active_route();

    html! {
        <div class="hidden lg:col-span-3 lg:block xl:col-span-2">
            <nav aria-label="Sidebar" class="sticky top-4 divide-y divide-gray-300">
                <div class="space-y-1 pb-8">
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
                </div>
            </nav>
        </div>
    }
}
