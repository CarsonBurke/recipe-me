#![allow(unused)]

use components::Navbar;
use dioxus::prelude::*;

mod components;
mod views;
// #[cfg(feature = "server")]
// mod server;

use dioxus_toast::ToastManager;
use views::{
    Home,
    login::Login,
    account::{
        dashboard::AccountDashboard,
    },
    recipe::{
        RecipePage, Recipes,
        recipes::{self, RecipeFilterParams},
    },
    signup::Signup,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    // #[cfg(feature = "server")]
    // tokio::runtime::Runtime::new()
    //     .unwrap()
    //     .block_on(server::launch_server());

    // #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/recipes?:..query")]
    Recipes {
        query: recipes::Query,
     },
    #[route("/recipe/:id")]
    RecipePage { id: i32 },
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[route("/dashboard")]
    AccountDashboard {}
}

#[component]
pub fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS },
        style { "{MAIN_CSS}" }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    let toast = use_context_provider(|| Signal::new(ToastManager::default()));

    rsx! {
        Navbar {}

        dioxus_toast::ToastFrame {
            manager: toast,
        }

        Outlet::<Route> {}
    }
}
