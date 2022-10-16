use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::icons::atom_icon::AtomIcon;
use crate::components::icons::molecule_icon::MoleculeIcon;
use crate::components::icons::organism_icon::OrganismIcon;
use crate::components::icons::home_icon::HomeIcon;
use crate::routes::AppRoute;
use log::{info, trace, warn};

pub fn isActiveRoute(path: String, expected: String) -> bool {
    path == expected
}

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    let active_route_class = "bg-gray-200 text-gray-900 group flex items-center px-3 py-2 text-sm font-medium rounded-md";
    let ddinactive_route_class = "text-gray-700 hover:bg-gray-50 group flex items-center px-3 py-2 text-sm font-medium rounded-md";
    let route: AppRoute = use_route().unwrap_or_default();
    let active_path = route.to_path();

    html! {
        <div class="hidden lg:col-span-3 lg:block xl:col-span-2">
            <nav aria-label="Sidebar" class="sticky top-4 divide-y divide-gray-300">
                <div class="space-y-1 pb-8">
                    <Link<AppRoute> to={AppRoute::Home}>
                        <a class={ if active_path == "/".to_string() { active_route_class } else { ddinactive_route_class } } aria-current="page">
                            // <HomeIcon classes={ "text-gray-400 group-hover:text-gray-500 flex-shrink-0 -ml-1 mr-3 h-6 w-6" } />
                            <i class="fa-regular fa-house-chimney-window fa-2xl text-gray-400 group-hover:text-gray-500 flex-shrink-0 -ml-1 mr-3"></i>
                            <span class="truncate">{"Home"}</span>
                        </a>
                    </Link<AppRoute>>

                    <Link<AppRoute> to={AppRoute::Atoms}>
                        <a class={ if active_path == "/atoms".to_string() { active_route_class } else { ddinactive_route_class } } aria-current="page">
                            <i class="fa-regular fa-atom-simple fa-2xl text-gray-400 group-hover:text-gray-500 flex-shrink-0 -ml-1 mr-3"></i>
                            <span class="truncate">{"Atoms"}</span>
                        </a>
                    </Link<AppRoute>>

                    <Link<AppRoute> to={AppRoute::Molecules}>
                        <a class={ if active_path == "/molecules".to_string() { active_route_class } else { ddinactive_route_class } }>
                            <i class="fa-regular fa-chart-network fa-2xl text-gray-400 group-hover:text-gray-500 flex-shrink-0 -ml-1 mr-3"></i>
                            <span class="truncate">{"Molecules"}</span>
                        </a>
                    </Link<AppRoute>>

                    <Link<AppRoute> to={AppRoute::Organisms}>
                        <a class={ if active_path == "/organisms".to_string() { active_route_class } else { ddinactive_route_class } }>
                            <i class="fa-regular fa-bacterium fa-2xl text-gray-400 group-hover:text-gray-500 flex-shrink-0 -ml-1 mr-3"></i>
                            <span class="truncate">{"Organisms"}</span>
                        </a>
                    </Link<AppRoute>>
                </div>
            </nav>
        </div>
    }
}
