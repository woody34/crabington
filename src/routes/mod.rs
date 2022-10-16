use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;
pub mod atoms;
pub mod molecules;
pub mod organisms;

use about::About;
use home::Home;
use atoms::Atoms;
use molecules::Molecules;
use organisms::Organisms;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[at("/atoms")]
    Atoms,
    #[at("/molecules")]
    Molecules,
    #[at("/organisms")]
    Organisms,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Atoms => html! { <Atoms /> },
        AppRoute::Molecules => html! { <Molecules /> },
        AppRoute::Organisms => html! { <Organisms /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
