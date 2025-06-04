use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;
use dioxus_primitives;

use crate::{components::{dialog::Dialog, filtered_recipes::FilteredRecipes}, views, Route};

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "column gapMedium",
                    h1 { class: "textLarge", "Your favourites" },
                    div {
                        class: "row overflowHorizontal gapMedium paddingLarge",
                        FilteredRecipes {}
                    }
                }
                div {
                    class: "column gapMedium",
                    h1 { class: "textLarge", "Quick access" },
                    div {
                        class: "row overflowHorizontal gapMedium paddingLarge",
                        Link {
                            class: "button buttonBg2 round square",
                            to: Route::Recipes { query: views::recipe::recipes::Query::default() },
                            dioxus_free_icons::Icon { icon: ld_icons::LdBook }
                            "My recipes"
                        }
                        Dialog {
                            button {
                                class: "button buttonBg2 round square",
                                dioxus_free_icons::Icon { icon: ld_icons::LdImport }
                                "Find recipe"
                                p { class: "textXSmall", "Import a recipe from online" }
                            }
                            button {
                                class: "button buttonBg2 round square",
                                dioxus_free_icons::Icon { icon: ld_icons::LdImport }
                                "Create recipe"
                                p { class: "textXSmall", "Make recipe from scratch" }
                            }
                        }
                        button {
                            class: "button buttonBg2 round square",
                            onclick: {
                                dioxus_primitives
                            },
                            dioxus_free_icons::Icon { icon: ld_icons::LdImport }
                            "New recipe"
                        }
                        /*
                        Link {
                            class: "button buttonBg2 round",
                            to: Route::AccountRecipes { query: recipes::Query::default() },
                            dioxus_free_icons::Icon { icon: ld_icons::LdHeart }
                            "Favourite recipes"
                        }
                        Link {
                            class: "button buttonBg2 round",
                            to: Route::AccountCollections {},
                            dioxus_free_icons::Icon { icon: ld_icons::LdSquareLibrary }
                            "My collections"
                        } */
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