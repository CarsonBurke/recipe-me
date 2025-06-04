use dioxus::prelude::*;

use views::{Blog, Home};

use crate::{components::navbar::Navbar, views::{recipe::{recipes::Recipes, recipe_page::RecipePage, new_recipe::NewRecipe}, dashboard::Dashboard, collection::{collections::Collections, new_collection::NewCollection}}};

mod views;
mod server;
mod components;
mod utils;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MobileNavbar)]
    #[route("/")]
    Dashboard {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/recipes?:..query")]
    Recipes {
        query: views::recipe::recipes::Query,
    },
    #[route("/recipe/:id")]
    RecipePage { id: i32 },
    #[route("/collections")]
    Collections {},
    #[route("/new_recipe")]
    NewRecipe {},
    #[route("/new_collection")]
    NewCollection {},
}

fn main() {
    dioxus::launch(App);
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const ANIMATIONS_CSS: Asset = asset!("/assets/styling/animations.css");

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS },
        style { "{MAIN_CSS} {ANIMATIONS_CSS}" }

        Router::<Route> {}
    }
}

/// A mobile-specific Router around the shared `Navbar` component
/// which allows us to use the mobile-specific `Route` enum.
#[component]
fn MobileNavbar() -> Element {
    rsx! {
        Navbar {  }

        Outlet::<Route> {}
    }
}
