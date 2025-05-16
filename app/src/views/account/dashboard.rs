use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{utils::is_logged_in, views::account::account_recipes, Route};

#[component]
pub fn AccountDashboard() -> Element {
    /* let is_logged_in = is_logged_in(); */

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "column gapMedium",
                    h1 { class: "textLarge", "Quick access" },
                    div {
                        class: "row overflowHorizontal gapMedium paddingLarge",
                        Link {
                            class: "button buttonBg2 round square",
                            to: Route::AccountRecipes { query: account_recipes::Query::default() },
                            dioxus_free_icons::Icon { icon: ld_icons::LdBook }
                            "My recipes"
                        }
                        Link {
                            class: "button buttonBg2 round",
                            to: Route::AccountRecipes { query: account_recipes::Query::default() },
                            dioxus_free_icons::Icon { icon: ld_icons::LdHeart }
                            "Favourite recipes"
                        }
                        Link {
                            class: "button buttonBg2 round",
                            to: Route::AccountCollections {},
                            dioxus_free_icons::Icon { icon: ld_icons::LdSquareLibrary }
                            "My collections"
                        }
                    }
                }
                div {
                    class: "column gapMedium",
                    h1 { class: "textLarge", "Actions"},
                    div {
                        class: "row overflowHorizontal gapMedium paddingLarge",
                        button {
                            class: "button buttonBg2 round square",
                            dioxus_free_icons::Icon { icon: ld_icons::LdImport }
                            "Import recipe"
                        }
                        button {
                            class: "button buttonBg2 round",
                            dioxus_free_icons::Icon { icon: ld_icons::LdCalendar }
                            "Meal plan"
                        }
                    }
                }
            }
        }
    }
}