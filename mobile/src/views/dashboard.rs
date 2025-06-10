use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{
    Route,
    components::{dialog::DialogWrapper, filtered_recipes::FilteredRecipes},
    views,
};

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
                        DialogWrapper {
                            header: rsx! {
                                h1 { class: "textLarge", "New Recipe" }
                            },
                            dialog: rsx! {
                                div {
                                    class: "row centerRow bg3 round",
                                    Link {
                                        to: Route::NewRecipe {  },
                                        class: "button buttonBg3 round square",
                                        div {
                                            class: "column gapXSmall centerColumn",
                                            div {
                                                class: "row centerRow gapSmall",
                                                dioxus_free_icons::Icon { icon: ld_icons::LdSearch }
                                                "Find recipe"
                                            }
                                            p { class: "textXSmall", "Import a recipe from online" }
                                        }
                                    }
                                    Link {
                                        to: Route::Recipes { query: views::recipe::recipes::Query::default() },
                                        class: "button buttonBg3 round square",
                                        div {
                                            class: "column gapXSmall centerColumn",
                                            div {
                                                class: "row gapSmall centerRow",
                                                dioxus_free_icons::Icon { icon: ld_icons::LdPlus }
                                                "Create recipe"
                                            }
                                            p { class: "textXSmall", "Make recipe from scratch" }
                                        }
                                    }
                                }
                            },
                            button: rsx! {
                                button {
                                    class: "button buttonBg2 round square",
                                    dioxus_free_icons::Icon { icon: ld_icons::LdBookPlus }
                                    "New recipe"
                                }
                            }
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
