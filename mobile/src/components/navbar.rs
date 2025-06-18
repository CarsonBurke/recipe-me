use dioxus::prelude::*;
use dioxus_free_icons::{
    self,
    icons::{self, ld_icons},
};

use crate::{views::recipe::recipes, Route};

const CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
            document::Link { rel: "stylesheet", href: CSS }

            nav {
                class: "navbar paddingSectionSide row bg3 gapSmall centerRow centerColumn",

                Link {
                    class: "width100 button buttonBg3 centerRow",
                    to: Route::Dashboard {},
                    div {
                        class: "column centerColumn centerRow",
                        p { class: "textSmall", dioxus_free_icons::Icon { icon: ld_icons::LdHome } }
                        p { class: "textXSmall", "Home" }
                    }
                }
                Link {
                    class: "width100 button buttonBg3 column centerRow textSmall",
                    to: Route::Recipes { 
                        query: recipes::Query::default(),
                    },
                    div {
                        class: "column centerColumn centerRow",
                        p { class: "textSmall", dioxus_free_icons::Icon { icon: ld_icons::LdBook } }
                        p { class: "textXSmall", "Recipes" }
                    }
                }
                Link {
                    class: "width100 button buttonBg3 column centerRow textSmall",
                    to: Route::Collections { public: false },
                    div {
                        class: "column centerColumn centerRow",
                        p { class: "textSmall", dioxus_free_icons::Icon { icon: ld_icons::LdBook } }
                        p { class: "textXSmall", "Collections" }
                    }
                }
                Link {
                    class: "width100 button buttonBg3 column centerRow textSmall",
                    to: Route::Settings {},
                    div {
                        class: "column centerColumn centerRow",
                        p { class: "textSmall", dioxus_free_icons::Icon { icon: ld_icons::LdSettings } }
                        p { class: "textXSmall", "Settings" }
                    }
                }
            }
        }
}
