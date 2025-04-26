use dioxus::prelude::*;

use crate::components::navbar::Navbar;

use crate::views::home::Home;
use crate::views::blog::Blog;
use crate::views::recipe::Recipe;
use crate::views::recipes::Recipes;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/recipes/")]
    Recipes {},
    #[route("/recipe/:id")]
    Recipe { id: i32 },
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
    rsx! {
        Navbar {
            Link {
                class: "buttonBg1 button",
                to: Route::Home {},
                "Home"
            }
            Link {
                class: "buttonBg1 button",
                to: Route::Recipes {},
                "Recipes"
            }
            Link {
                class: "buttonBg1 button",
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}