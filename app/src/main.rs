#![allow(unused)]

use api::{auth::ServerLoginToken, entities::prelude::LoginToken};
use components::Navbar;
use constants::LOGIN_TOKEN_KEY;
use dioxus::prelude::*;
use dioxus_motion::prelude::*;

mod components;
pub mod constants;
pub mod utils;
mod views;
// #[cfg(feature = "server")]
// mod server;

use dioxus_sdk::storage::{LocalStorage, use_synced_storage};
use dioxus_toast::ToastManager;
use views::{
    Home,
    account::{
        account::Account, account_recipes::AccountRecipes, collections::AccountCollections,
        dashboard::AccountDashboard,
        new_recipe::NewRecipe,
        new_collection::NewCollection,
    },
    collection::collection::CollectionPage,
    fallback::Fallback,
    login::Login,
    recipe::{
        RecipePage, Recipes,
        recipes::{self, RecipeFilterParams},
    },
    signup::Signup,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const ANIMATIONS_CSS: Asset = asset!("/assets/styling/animations.css");

fn main() {
    // Allows use to use local and session storage
    dioxus_sdk::set_dir!();

    // Try to assign the cached login token
    /* let cached_login_token = use_synced_storage::<LocalStorage, Option<ServerLoginToken>>(LOGIN_TOKEN_KEY.to_string(), || None);
    if let Some(cached_login_token) = cached_login_token() {
        *LOGIN_TOKEN_GLOBAL.write() = Some(cached_login_token);
    } */

    // #[cfg(feature = "server")]
    // tokio::runtime::Runtime::new()
    //     .unwrap()
    //     .block_on(server::launch_server());

    // #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[derive(Debug, Clone, Routable, PartialEq, MotionTransitions)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    #[transition(Fade)]
    Home {},
    #[route("/recipes?:..query")]
    #[transition(Fade)]
    Recipes {
        query: recipes::Query,
     },
    #[route("/recipe/:id")]
    #[transition(Fade)]
    RecipePage { id: i32 },
    #[route("/collection/:id")]
    #[transition(Fade)]
    CollectionPage { id: i32 },
    #[route("/login")]
    #[transition(Fade)]
    Login {},
    #[route("/signup")]
    #[transition(Fade)]
    Signup {},
    #[route("/account/dashboard")]
    #[transition(Fade)]
    AccountDashboard {},
    #[route("/account/recipes")]
    #[transition(Fade)]
    AccountRecipes {},
    #[route("/account/collections")]
    #[transition(Fade)]
    AccountCollections {},
    #[route("/account")]
    #[transition(Fade)]
    Account {},
    #[route("/account/new_recipe")]
    NewRecipe {},
    #[route("/account/new_collection")]
    NewCollection {},
    #[route("/:..route")]
    #[transition(Fade)]
    Fallback { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    let cached_login_token = use_synced_storage::<LocalStorage, Option<ServerLoginToken>>(
        LOGIN_TOKEN_KEY.to_string(),
        || None,
    );
    if let Some(cached_login_token) = cached_login_token() {
        *LOGIN_TOKEN_GLOBAL.write() = Some(cached_login_token);
    }

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS },
        style { "{MAIN_CSS}" }
        style { "{ANIMATIONS_CSS}" }


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

pub static LOGIN_TOKEN_GLOBAL: GlobalSignal<Option<ServerLoginToken>> = Signal::global(|| None);
