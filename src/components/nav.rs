use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::icons::atom_icon::AtomIcon;
use crate::components::icons::molecule_icon::MoleculeIcon;
use crate::components::icons::organism_icon::OrganismIcon;

use crate::routes::AppRoute;

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        // <nav>
        //     <ul>
        //         <li><Link<AppRoute> to={AppRoute::Home} classes="app-link" >{ "Home" }</Link<AppRoute>></li>
        //         <li><Link<AppRoute> to={AppRoute::About} classes="app-link">{ "About" }</Link<AppRoute>></li>
        //     </ul>
        // </nav>
        <div class="hidden lg:col-span-3 lg:block xl:col-span-2">
            <nav aria-label="Sidebar" class="sticky top-4 divide-y divide-gray-300">
                <div class="space-y-1 pb-8">
                    // <!-- Current: "bg-gray-200 text-gray-900", Default: "text-gray-700 hover:bg-gray-50" -->
                    <a href="#" class="bg-gray-200 text-gray-900 group flex items-center px-3 py-2 text-sm font-medium rounded-md" aria-current="page">
                        <AtomIcon classes={ "text-gray-400 group-hover:text-gray-500 flex-shrink-0 -ml-1 mr-3 h-6 w-6" } />
                        <span class="truncate">{"Atoms"}</span>
                    </a>

                    <a href="#" class="text-gray-700 hover:bg-gray-50 group flex items-center px-3 py-2 text-sm font-medium rounded-md">
                        <MoleculeIcon classes={ "text-gray-400 group-hover:text-gray-500 flex-shrink-0 -ml-1 mr-3 h-6 w-6" } />
                        <span class="truncate">{"Molecules"}</span>
                    </a>

                    <a href="#" class="text-gray-700 hover:bg-gray-50 group flex items-center px-3 py-2 text-sm font-medium rounded-md">
                        <OrganismIcon classes={ "text-gray-400 group-hover:text-gray-500 flex-shrink-0 -ml-1 mr-3 h-6 w-6" } />
                        <span class="truncate">{"Organisms"}</span>
                    </a>
                </div>
                // <div class="pt-10">
                //     <p class="px-3 text-sm font-medium text-gray-500" id="communities-headline">{"Communities"}</p>
                //     <div class="mt-3 space-y-2" aria-labelledby="communities-headline">
                //     <a href="#" class="group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
                //         <span class="truncate">{"Movies"}</span>
                //     </a>

                //     <a href="#" class="group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
                //         <span class="truncate">{"Food"}</span>
                //     </a>

                //     <a href="#" class="group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
                //         <span class="truncate">{"Sports"}</span>
                //     </a>

                //     <a href="#" class="group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
                //         <span class="truncate">{"Animals"}</span>
                //     </a>

                //     <a href="#" class="group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
                //         <span class="truncate">{"Science"}</span>
                //     </a>

                //     <a href="#" class="group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
                //         <span class="truncate">{"Dinosaurs"}</span>
                //     </a>

                //     <a href="#" class="group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
                //         <span class="truncate">{"Talents"}</span>
                //     </a>

                //     <a href="#" class="group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
                //         <span class="truncate">{"Gaming"}</span>
                //     </a>
                //     </div>
                // </div>
            </nav>
        </div>
    }
}
