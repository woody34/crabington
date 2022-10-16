use crate::routes::AppRoute;
use yew_router::*;
use yew_router::prelude::*;

pub fn use_active_route() -> String {
    let route: AppRoute = use_route().unwrap_or_default();
    route.to_path()
}