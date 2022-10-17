use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::components::header::Header;
use crate::routes::{switch, AppRoute};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-full">
                <Header />
                <div class="py-10">
                    <div class="mx-auto sm:px-2 lg:grid lg:grid-cols-12 lg:gap-8 lg:px-4">
                        <Nav />
                        <main class="lg:col-span-9 xl:col-span-9">
                            <div class="px-4 sm:px-0">
                                <Switch<AppRoute> render={Switch::render(switch)} />
                            </div>
                        </main>
                    </div>
                </div>
            </div>
        </BrowserRouter>
    }
}
