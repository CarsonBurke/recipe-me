use std::fmt::{self, Display};

use dioxus::{logger::tracing::info, prelude::*};

use dioxus_motion::prelude::*;
use dioxus_sdk::storage::{use_persistent, use_storage, use_synced_storage, LocalStorage};
use serde::{Deserialize, Serialize};

use crate::{
    components::navbar::Navbar, server::collection::test_find_collection, views::{
        collection::{collections::Collections, local::CollectionLocal, private::CollectionPrivate, new_collection::NewCollection},
        dashboard::Dashboard,
        recipe::{local::RecipeLocal, public::RecipePublic, new::NewRecipeView, recipes::Recipes},
        settings::{personalize::Personalize, premium::Premium, view::Settings},
    }
};

mod components;
mod constants;
mod server;
mod utils;
mod views;
mod entities;
mod data;

#[derive(Debug, Clone, Routable, PartialEq, MotionTransitions)]
#[rustfmt::skip]
enum Route {
    #[layout(MobileNavbar)]
    #[route("/")]
    #[transition(Fade)]
    Dashboard {},
    #[route("/recipes?:..query")]
    #[transition(Fade)]
    Recipes {
        query: views::recipe::recipes::Query,
    },
    #[route("/recipe_local/:id")]
    #[transition(Fade)]
    RecipeLocal { id: i32 },
    #[route("/recipe_public/:id")]
    #[transition(Fade)]
    RecipePublic { id: i32 },
    #[route("/collections")]
    #[transition(Fade)]
    Collections { public: bool },
    #[route("/new_recipe")]
    #[transition(Fade)]
    NewRecipeView {},
    #[route("/new_collection")]
    #[transition(Fade)]
    NewCollection {},
    #[route("/settings")]
    #[transition(Fade)]
    Settings {},
    #[route("/personalize")]
    #[transition(Fade)]
    Personalize {},
    #[route("/premium")]
    #[transition(Fade)]
    Premium {},
    #[route("/collection_local/:id")]
    #[transition(Fade)]
    CollectionLocal { id: i32 },
    #[route("/collection_private/:id")]
    #[transition(Fade)]
    CollectionPrivate { id: i32 },
}

fn main() {
    // Throwing weird error, don't know why; should be able to do use_resource and use_persistent
    /* let x = use_resource(|| async move {
        let is_db_initialized = use_persistent(constants::IS_DB_INITIALIZED.to_string(), || false);

        if !is_db_initialized() {
            
            server::init_db().await;
            use_persistent(constants::IS_DB_INITIALIZED.to_string(), || true);
        }

        println!("Completed coroutine");
        false
    });
    x(); */

    dioxus::launch(App);
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, enum_iterator::Sequence,
)]
pub enum Theme {
    #[default]
    Pastel,
    Midnight,
    White,
}

impl Theme {
    pub fn file_name(&self) -> String {
        match self {
            Theme::Pastel => "pastel".to_string(),
            Theme::Midnight => "midnight".to_string(),
            Theme::White => "white".to_string(),
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file_name())
    }
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match value.as_str() {
            "pastel" => Theme::Pastel,
            "midnight" => Theme::Midnight,
            "white" => Theme::White,
            _ => Theme::default(),
        }
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const ANIMATIONS_CSS: Asset = asset!("/assets/styling/animations.css");

// Themes
/* const THEME_MIDNIGHT: Asset = asset!("/assets/styling/themes/midnight.css");
const THEME_PASTEL: Asset = asset!("/assets/styling/themes/pastel.css");
const THEME_WHITE: Asset = asset!("/assets/styling/themes/white.css"); */

#[component]
fn App() -> Element {
    // Build cool things ✌️



    /* let cached_theme = use_persistent::<Theme>(constants::THEME.to_string(), || Theme::default());
    println!("cached theme {}", cached_theme());
    /* let theme_context = use_context_provider(|| use_signal(|| cached_theme()));
    print!("theme: {}", theme_context()); */

    *THEME_GLOBAL.write() = cached_theme();

    let mut theme: Signal<Asset> = use_signal(|| match cached_theme() {
        Theme::Pastel => THEME_PASTEL,
        Theme::Midnight => THEME_MIDNIGHT,
        Theme::White => THEME_WHITE,
    });

    let mut x = Theme::default()/* use_signal(|| Theme::default()) */; */

    /* use_effect(move || {
        /* cached_theme.read();
        THEME_GLOBAL.read(); */

        let z = THEME_GLOBAL();
        *&mut x = z.clone();

        println!("read signals");

        /* let read = THEME_GLOBAL.read(); */

        /* theme.set(match x {
            Theme::Pastel => THEME_PASTEL,
            Theme::Midnight => THEME_MIDNIGHT,
            Theme::White => THEME_WHITE,
        }); */
    }); */

    /* let collection = use_resource(move || async move { test_find_collection(1).await.unwrap() });
    print!("collection resource: {:#?}", collection()); */

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS },
        /* document::Link { rel: "stylesheet", href: {
            match x {
                Theme::Pastel => THEME_PASTEL,
                Theme::Midnight => THEME_MIDNIGHT,
                Theme::White => THEME_WHITE,
            }
        } },
        {format!("{}", x)} */
        document::Link { rel: "stylesheet", href: ANIMATIONS_CSS },

        Router::<Route> {}
    }
}

/// A mobile-specific Router around the shared `Navbar` component
/// which allows us to use the mobile-specific `Route` enum.
#[component]
fn MobileNavbar() -> Element {
    rsx! {
        div {
            class: "root theme_midnight",
            Navbar {  }

            AnimatedOutlet::<Route> {}
        }
    }
}

pub static THEME_GLOBAL: GlobalSignal<Theme> = Signal::global(|| Theme::default());
