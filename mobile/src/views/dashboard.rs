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
                    div {
                        class: "row gapMedium centerColumn spaceBetween",
                        h1 { class: "textLarge", "Your favourites" },
                        Link {
                            to: Route::Recipes { query: views::recipe::recipes::Query::default() },
                            class: "button buttonBg2",
                            "See all"
                        }
                    }
                    div {
                        class: "row overflowHorizontal gapMedium paddingLarge",
                        FilteredRecipes {
                            recipe_select: false,
                        }
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
                                    class: "row centerRow gapSmall round",
                                    Link {
                                        to: Route::Recipes { query: views::recipe::recipes::Query {
                                            recipe_select: true,
                                            ..Default::default()
                                        } },
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
                                        to: Route::NewRecipe {  },
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
                        Link {
                            class: "button buttonBg2 round",
                            to: Route::Collections {},
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
